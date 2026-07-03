# Hermes Tweet SkillHub Example

This example shows how to publish Hermes Tweet to a SkillHub registry and install it for Hermes Agent users.

Hermes Tweet provides X/Twitter research, reading, and gated action workflows through Xquik. Keep action access disabled until an operator explicitly approves write-capable workflows.

## Prepare The Package

Clone the public Hermes Tweet repository:

```bash
git clone https://github.com/Xquik-dev/hermes-tweet.git
cd hermes-tweet
```

The OpenSkills-style package directory is:

```text
hermes_tweet/skills/hermes-tweet/
```

## Publish

Point the CLI at your SkillHub registry and publish the package:

```bash
skillhub config set registry https://skillhub.your-company.com
skillhub publish ./hermes_tweet/skills/hermes-tweet --slug hermes-tweet --version 0.1.6
```

Use tags such as `hermes`, `x-twitter`, `social-media`, and `automation` if your registry supports tagging.

## Install

Search first, then install the published skill:

```bash
skillhub search hermes-tweet
skillhub install hermes-tweet
```

## Runtime Configuration

Do not put secret values in SkillHub docs or skill files. Configure them only in the Hermes runtime environment.

| Name | Purpose |
| --- | --- |
| `XQUIK_API_KEY` | Enables live read tools. |
| `HERMES_TWEET_ENABLE_ACTIONS` | Enables gated action tools only when set to `true`. |

Leave `HERMES_TWEET_ENABLE_ACTIONS` unset for research-only agents.
