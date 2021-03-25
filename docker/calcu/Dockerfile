# Build calcu image
FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install -y openssl
COPY calcu /opt/calcu/calcu
COPY run.sh /opt/run.sh

WORKDIR /opt/calcu/
CMD /opt/run.sh
