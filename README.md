# Experimentor
With experimentor you can create feature toggles. These feature toggles can either be toggled based on probability (between 0 and 100), or by segment (e.g. "beta-tester", etc.).

The focus of this application is on:

- **Performance**: since Experimentor is written in Rust with Actix, Experimentor is highly performant and doesn't suffer from latency due to random garbage collection events.
- **Scalability**: even though the same result is served if the user calls the same endpoint and the set of feature toggles is evenly distributed, Experimentor does not need a database for determining the features, but instead relies on a simple settings file. As such, Experimentor will work perfectly fine, no matter how many people use it.
- **Practicality**: simple, lightweight and dual licensed under the permissive Apache 2 or MIT license at your choice makes Experimentor practical for every organization.

# Example
Imagine you are the director of Pulp Fiction. You remastered the movie and want to test if the briefcase of Marcellus Wallace should have a silver or gold glow. The original movie had a gold glow, so you want to test if the silver glow would lead to a more enthusiastic audience.

Of course, you don't want each and every user to immediately see the silver glow, but maybe, say, 1% of the users, and some beta testers.

This use case would map to an `experiment.yml` file that would look as following:

```yaml
---
contexts:
  - name: pulp_fiction
    features:
      - name: briefcase
        description: A briefcase full of a certain product.
        treatments:
          - probability: 99
            value: gold
            segments: [ ]
          - probability: 1
            value: silver
            segments:
              - beta_testers
    segments:
      - name: beta_testers
        user_identifiers:
          - quentin
          - vincent_vega
          - butch
          - the_wolf
```
In this case, when the endpoint `/experiments/pulp_fiction`
is called, it will in 99% of the cases yield:
```
{
  "toggles": {
    "briefcase": "gold"
  }
}
```
And in 1% of the case and for users with identifiers (which can be anything, such as an IP address, a username, username hash) like `quentin_tarantino_1963`, `vincent_vega`, etc:
```
{
  "toggles": {
    "briefcase": "silver"
  }
}
```
As you may have noticed, the toggles are already mapped into objects, so they can be easily used by consumers such as Javascript.

# Usage
Starting the application with `experiments.yml` on port `8080`:
```
experimentor example.yml 8080
```
Or from the code:
```
cargo run example.yml 8080
```
You can visit `http://localhost:8080/feature-toggles/vincent_vega` or `http://localhost:8080/feature-toggles/someone_else` to see the response.

# In Progress...
For the program described above to actually work:
- Support multiple contexts.
- Verify probability logic.

# In a Future Release...
- Add OpenAPI specs.
- Add YAML Schema for the config.
