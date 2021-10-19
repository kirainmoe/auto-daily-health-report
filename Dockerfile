FROM python:3.7-buster
LABEL maintainer="CharlieJ107<charlie_j107@outlook.com>"
LABEL version="2021-10-20"
ENV XMU_USERNAME=""
ENV XMU_PASSWORD=""

ADD ./* /daily-report/
WORKDIR /daily-report

RUN pip config set global.index-url https://mirrors.cloud.tencent.com/pypi/simple; \
    pip install --no-cache-dir -r ./requirements.txt ;

CMD bash -c "while : ; do /usr/local/bin/python /daily-report/app.py $XMU_USERNAME $XMU_PASSWORD check; sleep 24h; done"