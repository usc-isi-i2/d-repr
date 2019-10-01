FROM continuumio/miniconda3:4.6.14

RUN apt-get update && apt-get install -y build-essential && pip install pipenv
RUN conda install -c conda-forge uwsgi

# add nginx:nginx user so that we can let uwsgi and nginx talk to each other
RUN groupmod -n nginx "$(getent group 101 | cut -d: -f1)" && useradd -u 101 -g 101 nginx

ADD ./webapp/Pipfile /webapp/Pipfile
RUN cd /webapp && pipenv lock && pipenv install --system --deploy

ADD ./pydrepr /webapp/pydrepr
RUN cd /webapp/pydrepr && pip install .

ADD ./webapp /webapp
ADD ./webapp/config/ontologies /webapp_home/ontologies

RUN cp /webapp/.env.template /webapp/.env

WORKDIR /webapp
