jq -r '.[] | select((.path + " " + .description) | test("parser"; "i")) | "\(.path): \(.description)"' code-index.json
