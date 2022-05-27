# Data Access Model

The data access model of the dataset is based on dataset's semantic model, and is similar to entity relation model. Specifically, we have classes, which contains entities. A class may have link to other classes and is uniquely identified by URI. An entity is also identified uniquely by its URI, and will have a set of properties that is defined by the class it belongs to.

Since the semantic model allows users to define more than one class that have same URI, this means all same type entities in the dataset may be split to multiple groups. For instance, a semantic model has two Person class: one is mentor and one is mentee (i.e. involve in different relationships), so that all people in the dataset can be split to two groups: mentors and mentees.

To summary, a data access model that has all of the above characteristics must have the following features:

1. Obtain id of an entity, which is:
    a. URI (string) if the entity is not blank.
    b. Any immutable, hashable value if the entity is blank.
2. Obtain the entity by id. User do not need to provide class id
    Implementation note:
    a. For graph backend, since the data is already in RDF, it doesn't require any special implementation
    b. For array backend, id of an entity is both URI and their index (need to inherit the str class)
    

