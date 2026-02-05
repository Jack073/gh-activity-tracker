Note: This is a WIP and exists largely to serve as a draft document for me to plan out the libraries, API's and documentation I will need to gather.

## Config File:
Consideration: Most of this could be wrapped by an API layer to allow us to use a database table instead for multiple
users - right now, config file will be fine.

YAML vs JSON

Needs to specify:
- Webhook URL
- Security Tokens
- Account Username
- Image location

## ENV Vars
database connection data

## WebServer
Server API - https://github.com/seanmonstar/warp

Basic dashboard for configuring visibility rules

Easiest way to store the image is to upload it to upload it to a file on GitHub and use permanent link in `{profile}/README`
Need to decide if we're going for the Cron job route, or wait x time after a commit arrives so we can batch.

Handle incoming webhook events
 - Validation & Authentication
 - Consider CF workers for validation to further reduce processing demand.

Things we need to track:
    - repository "edited" - default branch changes. If this happens it is probably worth dropping all commits from the 
        old branch and refetching.
    - push - has a list of commits, includes branch deletion, tag deletion & repository from template.
    - pull_request / issues - maybe have this as a TODO / nice to have.

## GH API
https://github.com/XAMPPRocky/octocrab
 - Provides type definitions for decoding webhook messages.
 - Provides API routes to run search on commits for user.
 - Considerations: 
   - search returns default branch only
   - octocrab has no rate limiting by default - searching will almost certainly require pagination

Runtime / async lib: tokio

SQL Library: tokio-postgres (async client)

# Implemented Features 
- [ ] Dashboard
  - [ ] GitHub OAuth to match user account with access token.
  - [ ] Dashboard allows whitelist / blacklist private repositories
  - [ ] Dashboard allows specific control, counting if specific repositories should be included or not
  - [ ] Image preview
- [ ] Fetching & storing historic data
- [ ] Webhook handling
  - [ ] Updating database from push events
  - [ ] Update repository visibility from repository edited events
  - [ ] Default branch changes
  - [ ] Track PRs
    - [ ] Open
    - [ ] Closed
    - [ ] Commented
    - [ ] Merged
  - [ ] Track Issues
    - [ ] Open
    - [ ] Closed
    - [ ] Commented
    - [ ] Resolved
  - [ ] Handle failed webhook events
- [ ] Publishing update activity maps