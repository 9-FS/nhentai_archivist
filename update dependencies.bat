poetry cache clear --all pypi
poetry update
poetry export -f requirements.txt -o requirements.txt --without-hashes

pause