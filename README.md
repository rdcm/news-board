## About

Cloud-native news management api built with Rust, leveraging gRPC, PostgreSQL, and Kubernetes for scalability and efficiency.

### UseCases::Auth

`SignUp` - signup and getting `session_id`  
`SignIn` - signin and getting `session_id`  
`SignOut` - invalidate current session

### UseCases::Articles

`GetArticle` - get article by id  
`GetArticles` - get page of articles (endless paging)  
`CreateArticle` - create article  
`DeleteArticle` - delete article  
`UpdateArticle` - update article

## Environment
`brew install libpq && brew link --force libpq` - installing `libpq` for interact with postgres  
`cargo install diesel_cli --no-default-features --features postgres` - installing `diesel_cli` for build diesel    

## UP & Running

### Local::Build
`make build-dev` - build locally  
`make format` - format code  
`make lint` - lint code  

### Local::Docker
`make docker-build` - build docker images  
`make docker-up` - up api and db  
`make docker-down` - down all containers

### Local::Kubernetes

`make deploy-k8s-dev` - deploy helm chart with values for local kubernetes cluster  
`make render-k8s-dev` - render helm chart with values for local kubernetes cluster  
`make delete-k8s` - delete helm chart  

