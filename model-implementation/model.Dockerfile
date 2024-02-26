FROM python:3

RUN pip install --upgrade pip

RUN adduser worker
USER worker
WORKDIR /app

RUN pip install --user pipenv

ENV PATH="/home/.local/bin:${PATH}"

RUN pip install --user flask requests numpy matplotlib scikit-learn pandas

COPY --chown=worker:worker ./model-implementation/model/ .

EXPOSE 5000

CMD ["python", "app.py"]
