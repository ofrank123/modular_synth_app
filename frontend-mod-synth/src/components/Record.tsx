import { useCallback, useEffect, useRef, useState } from "react";
import { useAudioContext } from "../hooks/audioContext";

export const RecordButton = () => {
  const { recorder } = useAudioContext();
  const [recording, setRecording] = useState(false);
  const [downloadData, setDownloadData] = useState<{
    href: string;
    download: string;
  } | null>(null);
  const chunks = useRef<Blob[]>([]);

  useEffect(() => {
    if (recorder) {
      recorder.ondataavailable = (e) => {
        chunks.current.push(e.data);
      };

      recorder.onstop = (e) => {
        const clipName = prompt("Enter a name for your clip");
        const blob = new Blob(chunks.current, {
          type: "audio/ogg; codecs=opus",
        });
        chunks.current = [];
        //@ts-ignore
        const blobUrl = window.URL.createObjectURL(blob);
        setDownloadData({
          download: clipName ?? "clip",
          href: blobUrl,
        });
        //@ts-ignore
        //window.URL.revokeObjectURL(blobUrl);
      };
    }
  }, [recorder]);

  useEffect(() => {
    const a = document.getElementById("download-link");
    a!.click();
  }, [downloadData]);

  const toggleRecording = useCallback(() => {
    if (recorder) {
      setRecording((oldRecording) => {
        if (!oldRecording) {
          if (recorder.state != "recording") {
            recorder.start();
            console.log("started recording");
          }

          return true;
        } else {
          if (recorder.state == "recording") {
            recorder.stop();
            console.log("stopped recording");
          }

          return false;
        }
      });
    }
  }, [recorder]);

  return (
    <>
      <button onClick={() => toggleRecording()}>
        {!recording ? "Start Recording" : "Stop Recording"}
      </button>
      <a id="download-link" style={{ display: "none" }} {...downloadData}>
        Download Clip
      </a>
    </>
  );
};
