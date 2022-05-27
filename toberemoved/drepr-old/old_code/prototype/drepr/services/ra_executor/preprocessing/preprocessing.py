import copy
from itertools import islice

from drepr.models import Representation
from drepr.models.preprocessing import TransformFunc, FuncType
from drepr.services.ra_executor.preprocessing.exec_func import compile_func, exec_func
from drepr.services.ra_executor.preprocessing.ground_location import ground_location, ground_variable_locations
from drepr.services.ra_reader.ra_reader import RAReader


def exec_preprocess_func(repr: Representation, func: TransformFunc, ra_reader: RAReader):
    loc = func.get_location(repr)
    loc = ground_location(ra_reader, repr, loc)

    if func.type == FuncType.Flatten:
        for index, values in ra_reader.iter_data(loc, reverse=True):
            ra_reader.replace_value(index, values[0])
            index[-1] += 1
            for value in reversed(values[1:]):
                ra_reader.insert_value(index, value)
            index[-1] -= 1
    else:
        func_args = compile_func(ra_reader, func.function)

        if func.type == FuncType.Map:
            for index, value in ra_reader.iter_data(loc):
                new_value = exec_func(value, index, func_args)
                ra_reader.replace_value(index, new_value)
        elif func.type == FuncType.Filter:
            for index, value in ra_reader.iter_data(loc, reverse=True):
                if not exec_func(value, index, func_args):
                    ra_reader.remove_value(index)
        elif func.type == FuncType.Split:
            # the last index should be an array because we are splitting an array based on its item
            # the first piece of an array cannot be empty (even if the split function return true for that
            slice = loc.slices[-1]

            if slice.is_range():
                # only split an array, if the step is not 1, we just dump to check different
                # elements
                ploc = loc.get_parent()
                if len(ploc.slices) == 0:
                    # at the root, we have to handle separately as we don't have parent
                    new_values = [[]]
                    # TODO: avoid copy index every time.
                    indexes = []
                    if loc.slices[0].step == 1:
                        data_iter = iter(ra_reader.iter_data(loc))
                        try:
                            index, value = next(data_iter)
                            new_values[-1].append(value)
                            indexes.append(copy.copy(index))
                            func_args[1]['__prev_value__'] = value
                        except StopIteration:
                            pass

                        for index, value in data_iter:
                            if exec_func(value, index, func_args):
                                new_values.append([])
                            new_values[-1].append(value)
                            indexes.append(copy.copy(index))
                            func_args[1]['__prev_value__'] = value
                    else:
                        cloc = loc.clone()
                        cloc.slices[0].step = 1
                        step = loc.slices[0].step

                        data_iter = iter(ra_reader.iter_data(cloc))
                        try:
                            index, value = next(data_iter)
                            new_values[-1].append(value)
                            indexes.append(copy.copy(index))
                            func_args[1]['__prev_value__'] = value
                        except StopIteration:
                            pass

                        for index, value in data_iter:
                            if index[-1] % step == 0:
                                if exec_func(value, index, func_args):
                                    new_values.append([])
                            new_values[-1].append(value)
                            indexes.append(copy.copy(index))
                            func_args[1]['__prev_value__'] = value

                    for idx in reversed(indexes[len(new_values):]):
                        ra_reader.remove_value(idx)

                    for idx, value in zip(indexes, new_values):
                        ra_reader.replace_value(idx, value)
                else:
                    if slice.step > 1:
                        for index, value in ra_reader.iter_data(ploc):
                            new_values = [[value[slice.start]]]
                            index.append(slice.start)
                            for i in range(slice.start + 1, slice.end):
                                index[-1] = i
                                if i % slice.step == 0:
                                    if exec_func(value[i], index, func_args):
                                        new_values.append([])
                                new_values[-1].append(value[i])
                                func_args[1]['__prev_value__'] = value[i]

                            index[-1] = slice.start + len(new_values)
                            for i in range(slice.end - slice.start - len(new_values)):
                                ra_reader.remove_value(index)

                            for i, new_val in zip(range(slice.start, slice.start + len(new_values)), new_values):
                                index[-1] = i
                                ra_reader.replace_value(index, new_val)

                            index.pop()
                    else:
                        for index, value in ra_reader.iter_data(ploc):
                            new_values = [[value[slice.start]]]
                            index.append(slice.start)
                            for i in range(slice.start + 1, slice.end):
                                index[-1] = i
                                if exec_func(value[i], index, func_args):
                                    new_values.append([])
                                new_values[-1].append(value[i])
                                func_args[1]['__prev_value__'] = value[i]

                            index[-1] = slice.start + len(new_values)
                            for i in range(slice.end - slice.start - len(new_values)):
                                ra_reader.remove_value(index)

                            for i, new_val in zip(range(slice.start, slice.start + len(new_values)), new_values):
                                index[-1] = i
                                ra_reader.replace_value(index, new_val)

                            index.pop()
        else:
            raise NotImplementedError()


def exec_preprocessing(repr: Representation, ra_reader: RAReader):
    # execute preprocessing function
    for func in repr.preprocessing:
        exec_preprocess_func(repr, func, ra_reader)

    # grounding variable locations
    ground_variable_locations(ra_reader, repr)
