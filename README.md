# Real-Time DoS Detection: A ML-based system to combat DoS attacks.

## Description
This repository contains the definition of a simulated infrastructure consisting of three main elements: an e-commerce website (OWASP Juice Shop), a reverse proxy with WAF functionalities, and an offline ML model. The project aims to develop an ML model capable of learning from past traffic to establish our traffic baseline and generate thresholds to be provided to the active component (reverse proxy) via an available API. These thresholds would then be used to block potential DoS attacks in real-time.

The dataset used to train the model is the following: https://dataverse.harvard.edu/dataset.xhtml?persistentId=doi:10.7910/DVN/3QBYB5

## How to run
For all components of the system, a Docker image has been created and they were then configured in a single Docker Compose.
Therefore, to bring up the infrastructure, it is enough to execute the following command:

` ` ` bash
docker compose up 
` ` `

The infrastructure exposes the following ports:
* 8080: the e-commerce site.
* 5000: an interface for internal use to interact directly with the reverse proxy.
* 5001: an interface to interact with the Machine Learning model (to perform training and send new metrics to the reverse proxy).







