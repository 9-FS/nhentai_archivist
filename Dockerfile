ARG PYTHON_VERSION="3.12"
FROM python:$PYTHON_VERSION

ENV PYTHON_VERSION="3.12"


WORKDIR "/app/"
COPY . .

RUN python${PYTHON_VERSION} -m pip install poetry
RUN poetry config virtualenvs.in-project true
RUN poetry config repositories.test-pypi "https://test.pypi.org/legacy/"
RUN poetry install


CMD poetry run python${PYTHON_VERSION} "./src/main_outer.py"


# MANUAL BUILD:

# build docker image, @L to lowercase
# IMAGE_NAME="9-FS/2021-11-15-nHentai-to-PDF:latest" && docker build -t "${IMAGE_NAME@L}" --no-cache . && docker save "${IMAGE_NAME@L}" > "image.tar"

# on deployment environment load docker image from tar file
# docker load < "/mnt/user/appdata/image.tar"