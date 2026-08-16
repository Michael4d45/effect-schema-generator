<?php

declare(strict_types=1);

namespace App\Enums;

enum QuestionType: string
{
    use EnumUtil;

    case Artist = 'artist';
    case Title = 'title';
    case Year = 'year';
    case MultipleChoice = 'multiple_choice';
    case Lyric = 'lyric';
    case AudioClip = 'audio_clip';
}
