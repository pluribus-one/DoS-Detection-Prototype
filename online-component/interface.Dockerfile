FROM python:3

RUN pip install --upgrade pip

RUN adduser worker
USER worker
WORKDIR /app

RUN pip install --user pipenv

ENV PATH="/home/.local/bin:${PATH}"

RUN pip install --user flask Flask-WTF WTForms requests

COPY --chown=worker:worker ./online-component/interface/ .

EXPOSE 5000

CMD ["python", "/app/server.py"]
