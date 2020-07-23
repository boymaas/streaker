FROM openjdk:14-slim

# for example: https://api.opescash.com/api/v1/anode/attribution 
ENV STREAKER_END_POINT  https://api.opescash.com/api/v1/anode/attribution 
# for example: opesuntie
ENV STREAKER_ANODE_NAME opesunite

COPY app /app
COPY post_attribution /app
WORKDIR /app

# Install all the deps
RUN apt-get update && \
    apt-get install -y jq  && \
    rm -rf /var/lib/apt/lists/*

# post_attribution prints every event received to STDERR as other scripts
# are dependent on it. With buffering disabled. We merge them back into stdout
# to make sure the dependent scripts will take them.
CMD ./start.sh | ./post_attribution $STREAKER_ANODE_NAME $STREAKER_END_POINT 2>&1
