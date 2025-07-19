# TODO

These are personal TODOs, if you'd like to contribute to the project, please do not work on these features; they are also a way for me to learn more things. Many many thanks :D

## Tool
- [ ] Find open ports
- [ ] Find potential valuable files
- [ ] Enumerate emails for newly discovered domains
- [ ] Support more search engines and dorking methods
- [ ] Overall SSL scanner -> Is the certificate trusted, is it vulnerable to Heartbleed, etc.
- [ ] Trigger a webhook for the results, maybe provide support for specific webhooks e.g. Slack
- [ ] Have a core command like now that takes the config file, but also have seperate commands and its relevant configuration, e.g. `project-absence --domain <your-domain> dork --search-engine ecosia`
  - This will likely require some cleanup in the modules themselves and instead of returning `Result<(), String>` when executing a module, it would return the (list?) of `events::Type` to emit, or keep it as now but have a seperate crate or module to perform the actions, like check a port

## Website
- [ ] Roadmap of future goals (remove this file and put TODOs there as well?)
