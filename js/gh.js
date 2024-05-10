import { Octokit, App } from "octokit";

import 'dotenv/config'

// Create a personal access token at https://github.com/settings/tokens/new?scopes=repo
const octokit = new Octokit({ auth: process.env.GITHUB_TOKEN });

// Compare: https://docs.github.com/en/rest/reference/users#get-the-authenticated-user
const {
  data: { login },
} = await octokit.rest.users.getAuthenticated();

async function get_repos(from, to) {
  const {
    user: {
      contributionsCollection: {
        commitContributionsByRepository
      }
    }
  } = await octokit.graphql(
    `
    query userInfo($login: String!, $from: DateTime!, $to: DateTime!) {
        user(login: $login) {
          name
          contributionsCollection(from: $from, to: $to) {
            commitContributionsByRepository {
              repository {
                nameWithOwner
              }
            }
          }
        }
      }
    `,
    {
      login: login,
      from: from,
      to: to
    },
  );
  return commitContributionsByRepository.map(({ repository }) => repository.nameWithOwner)
}

let repos = new Array();
let to = new Date();
for (let i = 0; i < 10; i++) {
  let from = new Date(to);
  from.setFullYear(to.getFullYear() - 1);

  repos.push(await get_repos(from, to));
  to = from;
}

console.log(JSON.stringify(repos, null, 2))