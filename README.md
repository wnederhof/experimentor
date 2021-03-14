# Experimentor
Experimentor allows you to create feature toggles. These feature toggles can either be toggled based on probability (between 0 and 100), or by segment (e.g. "beta-tester", etc.). The program can be run from a simple yaml file, is extremely performant, and does not require a database.

The focus of this application is on:

- **Performance**: since Experimentor is written in Rust with Actix, Experimentor is highly performant and doesn't suffer from latency due to random garbage collection events.
- **Scalability**: even though the same result is served if the user calls the same endpoint and the set of feature toggles is evenly distributed, Experimentor does not need a database for determining the features, but instead relies on a simple settings file. As such, Experimentor will work perfectly fine, no matter how many people use it.
- **Practicality**: simple and lightweight.

**NOTE:** This project was created as an exercise to learn the Rust programming language. While the program may be useful for some of your purposes, currently there are no big plans for this project. More than anything, the project may be a great way to learn more about Rust. :-)

# Example
Imagine you are the director of Pulp Fiction. You remastered the movie and want to test if the briefcase of Marcellus Wallace should have a silver or gold glow. The original movie had a gold glow, so you want to test if the silver glow would lead to a more enthusiastic audience.

Of course, you don't want each and every user to immediately see the silver glow, but maybe, say, 1% of the users, and some beta testers.

This use case would map to an `experiment.yml` file that would look as following:

```yaml
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
In this case, when the endpoint `/contexts/pulp_fiction/feature-toggles/mia`
is called, it will in 99% of the cases yield:
```
{
  "hash": "cS6RYT8M3Bo=",
  "status": "OK",
  "toggles": {
    "briefcase": "gold"
  }
}
```
And in 1% of the case and for users with identifiers (which can be anything, such as an IP address, a username, username hash) like `quentin`, `vincent_vega`, etc:
```
{
  "toggles": {
    "hash": "m+oyVfDL3yc=",
    "status": "OK",
    "briefcase": "silver"
  }
}
```
And when the hash is provided, we can save some bandwidth, e.g.: `/contexts/pulp_fiction/feature-toggles/jamie_oliver?hash=cS6RYT8M3Bo%3D`
```
{
  "status": "CACHE_OK"
}
```
Yet, when we point to the wrong context, e.g. `/contexts/big_lebowski/feature-toggles/his_dudeness`, we get:
```
{
  "status": "NOT_FOUND"
}
```

# Usage
Starting the application with `experiments.yml` on port `8080`:
```
experimentor example.yml 8080
```
Or from the code:
```
cargo run example.yml 8080
```

# Health Checks
Experimentor is designed for use in cloud services. When using Experimentor in Kubernetes, for instance, you can provide the following endpoints for the liveness and readiness probe, so that you can do a rolling restart when the settings are updated:
```
/health
```
All this endpoint will ever do is respond with a 200 status code when the server is ready to serve requests.

# Performance
Did I say... Performant? A quick Apache Bench on an M1 Apple Macbook Air with 128 concurrent connections tells us that Experimentor is... Not slow:

```
$ ab -n 5000 -c 128 http://127.0.0.1:8080/contexts/pulp_fiction/feature-toggles/quentin
...
Time per request:       4.689 [ms] (mean)
Time per request:       0.037 [ms] (mean, across all concurrent requests)
Transfer rate:          4745.54 [Kbytes/sec] received
...
```
Yes, that is 0.037 milliseconds, or 37 microseconds per request. Sweet!


# In Progress
For the program described above to actually work:
- Verify probability logic. It may, or it may not be, correct. Requires proper investigation.

# Future Work
- Add OpenAPI specs.
- Add YAML Schema for the config.
- Add a Helm chart.
