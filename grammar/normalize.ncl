# This script flattens the Precursor config into the Intermediate format
# required by the Ada parsers.

let precursor = import "../loom.mittens.ncl" in

# 1. Helper function to transform a single contributor record
let transform = fun contributor_name contributor_data => {
    name = contributor_name,
    role = contributor_data.role,
    sandbox = contributor_data.sandbox,
    care_mode = contributor_data.care_mode,
    badge_goals = contributor_data.badge_goals,
    salt_states = contributor_data.salt.states,
    grains = contributor_data.salt.grains,
    shells = contributor_data.init.shell,
} in

# 2. Iterate over the contributor record and apply the transform
let flattened_contributors =
    precursor.contributor
    | Record.map transform
    | Record.values
in

# 3. Output the final Intermediate structure
{
    contributors = flattened_contributors
}
