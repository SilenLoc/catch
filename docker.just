image_name := "silenloc/catch"

@run d="-d": build stop
    docker run {{d}} --name catch -p 8111:8111 {{image_name}}

@stop:
    -docker stop catch
    -docker rm catch

build version="latest":
    docker build -t {{image_name}}:{{version}} .

push:
    just docker _d_push $(just docker get_version)

_d_push version:
    docker build -t {{image_name}}:{{version}} .
    docker push {{image_name}}:{{version}}

# Build and push with version tag and latest tag
release version:
    docker build -t {{image_name}}:{{version}} .
    docker push {{image_name}}:{{version}}
    docker tag {{image_name}}:{{version}} {{image_name}}:latest
    docker push {{image_name}}:latest

get_version:
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].version'
