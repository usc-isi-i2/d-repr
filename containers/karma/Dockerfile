FROM ubuntu:16.04

RUN apt-get update && \
	apt-get install -y wget software-properties-common openjdk-8-jdk

RUN	wget https://github.com/usc-isi-i2/Web-Karma/archive/v2.2.tar.gz && \
	tar -xf v2.2.tar.gz && \
	wget http://mirrors.koehn.com/apache/maven/maven-3/3.3.9/binaries/apache-maven-3.3.9-bin.tar.gz && \
	tar -xf apache-maven-3.3.9-bin.tar.gz && \
	rm v2.2.tar.gz apache-maven-3.3.9-bin.tar.gz

ENV PATH /apache-maven-3.3.9/bin:$PATH
ENV JAVA_HOME /usr/lib/jvm/java-8-openjdk-amd64

RUN mv /Web-Karma-2.2 /Web-Karma

RUN cd /Web-Karma && mvn clean install -Dmaven.test.skip=true
# jetty package is not installed by mvn install
RUN cd /Web-Karma/karma-web && mvn jetty:help

# install offline mode
RUN cd /Web-Karma/karma-offline && mvn install -P shaded

RUN cd /Web-Karma/karma-web-services/web-services-rdf && mvn install

# max perm size 128m was remove in java 8
ENV MAVEN_OPTS "-Xmx4096m"
ADD entrypoint.sh /

ENTRYPOINT ["bash", "entrypoint.sh"]