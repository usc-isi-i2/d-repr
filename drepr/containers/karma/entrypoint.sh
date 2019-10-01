#!/bin/bash

cmd=$1
shift 1

if [ "$cmd" == "web-karma" ]; then
    cd /Web-Karma/karma-web
    mvn jetty:run
elif [ "$cmd" == "web-services" ]; then
    cd /Web-Karma/karma-web-services/web-services-rdf
    mvn jetty:run
else
    echo "Incorrect command"
    exit 1
fi