**Prerequisite**

- Docker must be installed/running
- SAM must be installed


**Build**

`make prebuild`

**Deployment**

`make deploy`

Details:
- Lambda function needs to have a layer with the GS Shared Objects placed into the lib folder (so that they end up under /opt/lib once unpacked by AWS). This may be a manual step to upload the zipped /lib folder, and add it to the Lambda function
- the Lambda function has just one single executable called `bootstrap`