# Lambda function deploy script.
# From https://github.com/blombard/lambda-monorepo

FUNCTION_NAME=$1
PATH_NAME=$2
ZIP_PARAMS=$3

if [ -n "$PATH_NAME" ]; then cd $PATH_NAME; fi

zip lambda.zip -r $ZIP_PARAMS

aws lambda update-function-code --function-name $FUNCTION_NAME --zip-file fileb://lambda.zip
aws lambda update-function-configuration --function-name $FUNCTION_NAME --environment Variables="{`cat .env | xargs | sed 's/ /,/g'`}"

rm -f lambda.zip

exit 0
