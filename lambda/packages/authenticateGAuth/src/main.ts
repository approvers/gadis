import isEven from "is-even";
import {APIGatewayEventRequestContext, APIGatewayProxyEvent, APIGatewayProxyResult} from "aws-lambda";

export async function handler(
  event: APIGatewayEventRequestContext,
  context: APIGatewayProxyEvent
): Promise<APIGatewayProxyResult> {

  const randomNumber = Math.floor(Math.random() * 1000000);

  const response = {
    "message": "Beep-poop, here is authenticateGAuth.",
    "additional": {
      "number": randomNumber,
      "even": isEven(randomNumber)
    },
  };

  return {
    statusCode: 200,
    body: JSON.stringify(response)
  };
}
