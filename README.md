**Prerequisite**

- Docker must be installed/running
- SAM must be installed
- Ghostscript shall be installed and available in the $PATH

**Build**

1- The first time when the project is setup. It install all required libs in a base image (Cargo, C compiler, Ghotscript libs, etc.)
`make prepare-image`

2- Extract libs from 1. This is one time step and used as a Lambda Layer
`make get-ghostscript-libs`

3- To pre-build the cache so that each incremental build will be fast. It downloads crates, and build the cache
`make build-cache-fr` (replace `fr` by over apps with the right suffix)

4- Based on 3-, for incremental build
`make build-fr` 

**Deployment**

*Local dev*:
`make local`

Build steps are not required, but prerequisites are.

*Deployment to AWS*
`make deploy`

This is assumed that `prepare-image` along `build-cache` have already been run. `make deploy` will rebuild assets

**Details**
- Lambda function needs to have a layer with the GS Shared Objects placed into the lib folder (so that they end up under /opt/lib once unpacked by AWS). 
- the Lambda function has just one single executable called `bootstrap`