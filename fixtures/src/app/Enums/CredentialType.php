<?php

declare(strict_types=1);

namespace App\Enums;

enum CredentialType: string
{
    use EnumUtil;

    case ApiKey = 'api_key';
    case OauthToken = 'oauth_token';
    case ClientSecret = 'client_secret';
}
