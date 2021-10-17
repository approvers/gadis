import {APIGatewayEventRequestContext, APIGatewayProxyEvent, APIGatewayProxyResult} from "aws-lambda";

export async function handler(
  event: APIGatewayEventRequestContext,
  context: APIGatewayProxyEvent
): Promise<APIGatewayProxyResult> {

  const response = {
    "message": "Beep-poop, here is fetchDiscordUserInformation."
  };

  return {
    statusCode: 200,
    body: JSON.stringify(response)
  };
}
