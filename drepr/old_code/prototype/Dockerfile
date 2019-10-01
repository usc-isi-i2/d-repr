FROM continuumio/miniconda3:4.5.12

RUN apt-get update && apt-get install -y build-essential && pip install pipenv

ADD Pipfile /drepr/Pipfile
ADD Pipfile.lock /drepr/Pipfile.lock

# ERROR
#RUN cd /drepr && pipenv install --system --deploy
RUN pip install ujson pyyaml networkx netcdf4
RUN apt install -y tree

ADD drepr /drepr
RUN python -OO -m compileall -b /drepr && cd /drepr && find . -name "*.py" -delete

ENV PYTHONPATH /
ENTRYPOINT ["python", "/drepr/main.pyc"]
