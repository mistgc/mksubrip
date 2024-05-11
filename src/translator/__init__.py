from enum import Enum
from typing import Literal

class ModelScale(Enum):
    Tiny = 0
    Small = 1
    Base = 2
    Large = 3
    Specified = 4

    @staticmethod
    def from_str(scale:str):
        match str:
            case "tiny":
                return ModelScale.Tiny
            case "small":
                return ModelScale.Small
            case "base":
                return ModelScale.Base
            case "large":
                return ModelScale.Large
        # TODO: case Specified

        return ModelScale.Base

class ModelType(Enum):
    OpenaiWhisper = 0
