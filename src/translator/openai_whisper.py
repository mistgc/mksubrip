from typing import Any, Dict, List, Optional
import whisper
from translator import ModelScale
from translator import ModelType

class OpenaiWhisper:
    def __init__(self, scale: ModelScale):
        self.model_type = ModelType.OpenaiWhisper
        self.scale: ModelScale = scale
        self.model = None

    def init(self):
        scale_str = self._scale()
        if scale_str is not None:
            self.model = whisper.load_model(scale_str)

    def _scale(self) -> Optional[str]:
        match self.scale:
            case ModelScale.Tiny:
                return "tiny"
            case ModelScale.Small:
                return "small"
            case ModelScale.Base:
                return "base"
            case ModelScale.Large:
                return "large"

    def translate(self, file_path: str) -> List[Dict[str, Any]]:
        if self.model is not None:
            result = self.model.transcribe(file_path)
            segments = result["segments"]
            if isinstance(segments, str):
                return [{
                    "id" : 0,
                    "start" : 0.0,
                    "end" : 0.0,
                    "text" : segments
                }]
            elif segments is None:
                return []
            else:
                return segments
        else:
            return []

    @staticmethod
    def convert_to_srt_subtitle(segments: List[Dict[str, Any]]) -> str:
        import io
        from whisper.utils import format_timestamp

        string = io.StringIO("")
        for i, segment in enumerate(segments, start=1):
            index = i
            start = format_timestamp(segment["start"])
            end = format_timestamp(segment["end"])
            text = segment["text"]
            string.write(f"{index}\n{start} --> {end}\n{text}\n\n")


        return string.getvalue()
