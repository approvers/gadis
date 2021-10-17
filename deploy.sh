# Lambda function deploy script.
# From https://github.com/blombard/lambda-monorepo

FUNCTION_NAME=$1
PATH_NAME=$2

echo "==> Deploying $FUNCTION_NAME (at $PATH_NAME)"

if [ -n "$PATH_NAME" ]; then cd $PATH_NAME; fi

echo "    ==> Installing dependencies"

echo "    ==> Creating layer ZIP file"
zip lambda.zip -r dist/main.js

export AWS_LAMBDA_DEPLOY_FAILURE=0

echo "    ==> Deploying code"
aws lambda update-function-code --function-name $FUNCTION_NAME --zip-file fileb://lambda.zip || export AWS_LAMBDA_DEPLOY_FAILURE=1

echo "    ==> Deploying environment variables"
aws lambda update-function-configuration --function-name $FUNCTION_NAME --environment Variables="{`cat .env | xargs | sed 's/ /,/g'`}" || export AWS_LAMBDA_DEPLOY_FAILURE=1

rm -f lambda.zip

exit $AWS_LAMBDA_DEPLOY_FAILURE
