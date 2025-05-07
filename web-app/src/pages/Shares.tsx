import { useState } from "react";
import { postShares, splitPreferences } from "./api";

interface UserPreferences {
  id: string;
  age: number;
  gender: number;
  interests: number[];
  region: number;
  preferences: {
    age_min: number;
    age_max: number;
    gender: number;
  };
}

interface SharesProps {
  preferences: UserPreferences;
  getMatches: () => void;
}

export default function Shares({ preferences, getMatches }: SharesProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [isGenerating, setIsGenerating] = useState(false);
  const [uploadComplete, setUploadComplete] = useState(false);
  const [generateError, setGenerateError] = useState<string | null>(null);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [twitterHandle, setTwitterHandle] = useState("");
  const [shares, setShares] = useState<Uint8Array[]>([]);

  const generateShares = async () => {
    setIsGenerating(true);
    setGenerateError(null);
    try {
      const response_data = await splitPreferences(preferences);
      console.log("response_data", response_data);

      // Convert the response data to Uint8Array shares
      const newShares = response_data.shares.map((share: string) => Uint8Array.from(Buffer.from(share, "hex")));
      setShares(newShares);
    } catch (error) {
      console.error(error);
      setGenerateError("Failed to generate shares. Please try again.");
    } finally {
      setIsGenerating(false);
    }
  };

  const uploadShares = async () => {
    if (!shares.length) return;
    if (!twitterHandle) {
      setUploadError("Please enter your Twitter handle");
      return;
    }

    setIsUploading(true);
    setUploadError(null);

    try {
      const formData = new FormData();

      // Add each share as a separate file
      shares.forEach((share, index) => {
        const blob = new Blob([share], { type: "application/octet-stream" });
        formData.append(`file${index}`, blob);
      });

      const params = new URLSearchParams({
        twitter_handle: twitterHandle,
      });

      const data = await postShares(params, formData);
      console.log("DATA:", data);

      if (data.error) {
        setUploadError(data.error);
      } else {
        // Store the token in localStorage
        if (data.token) {
          localStorage.setItem("co-match-token", data.token);
        }
        setUploadComplete(true);
      }
    } catch (error) {
      console.error(error);
      setUploadError("Failed to upload shares. Please try again.");
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="bg-white/90 backdrop-blur-md rounded-2xl shadow-lg p-8 border border-pink-100">
      <h2 className="text-3xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent mb-6">
        Let&apos;s find you a match! 💝
      </h2>

      <div className="text-sm text-gray-600 mb-8 p-4 bg-gradient-to-r from-pink-50 to-purple-50 rounded-xl border border-pink-100">
        There are multiple steps to finding a match:
        <ol className="list-decimal list-inside space-y-2 mt-2">
          <li>First, you need to split your preferences into 3 encrypted shares.</li>
          <li>Then, you&apos;ll upload your shares to the network.</li>
          <li>Finally, you&apos;ll be able to view your matches.</li>
        </ol>
      </div>

      <div className="space-y-8">
        <div className="border border-gray-200 rounded-xl p-6 hover:border-purple-300 transition-all">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-xl font-semibold text-gray-900">Step 1: Generate Shares</h3>
              <p className="text-sm text-gray-500">Generate your encrypted preference shares</p>
            </div>
            {shares.length > 0 && <span className="text-green-600 text-sm font-medium">✓ Generated</span>}
          </div>

          {generateError && <div className="mb-4 p-4 bg-red-50 text-red-700 rounded-xl text-sm border border-red-100">{generateError}</div>}

          <button
            onClick={generateShares}
            disabled={isGenerating || shares.length > 0}
            className={`w-full py-3 px-6 rounded-xl transition-all duration-300 ${
              isGenerating || shares.length > 0
                ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                : "bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
            }`}
          >
            {isGenerating ? "Generating..." : shares.length > 0 ? "Shares Generated" : "Generate Shares"}
          </button>
        </div>

        <div className="border border-gray-200 rounded-xl p-6 hover:border-purple-300 transition-all">
          <div className="mb-4">
            <h3 className="text-xl font-semibold text-gray-900">Step 2: Upload Shares</h3>
            <p className="text-sm text-gray-600 mb-4">
              Twitter handler: To allow your matches to contact you, please enter your Twitter handle.
            </p>
            <div className="flex items-center space-x-2 mb-4">
              <span className="text-gray-500">@</span>
              <input
                type="text"
                value={twitterHandle}
                onChange={(e) => setTwitterHandle(e.target.value.replace("@", ""))}
                placeholder="your_twitter_handle"
                className="flex-1 px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all hover:border-purple-300"
              />
            </div>
            <div className="flex items-center justify-between mb-4">
              <p className="text-sm text-gray-500">Upload your preference shares to the network</p>
              {uploadComplete && <span className="text-green-600 text-sm font-medium">✓ Complete</span>}
            </div>
            {uploadError && <div className="mb-4 p-4 bg-red-50 text-red-700 rounded-xl text-sm border border-red-100">{uploadError}</div>}

            <button
              onClick={uploadShares}
              disabled={isUploading || uploadComplete || !shares.length || !twitterHandle}
              className={`w-full py-3 px-6 rounded-xl transition-all duration-300 ${
                isUploading || uploadComplete || !shares.length || !twitterHandle
                  ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                  : "bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
              }`}
            >
              {isUploading ? "Uploading..." : uploadComplete ? "Uploaded" : "Upload Shares"}
            </button>
          </div>
        </div>

        <div className="border border-gray-200 rounded-xl p-6 hover:border-purple-300 transition-all">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-xl font-semibold text-gray-900">Step 3: Match!!! 💖</h3>
              <p className="text-sm text-gray-500">Process your encrypted profile to find matches</p>
            </div>
          </div>

          <button
            onClick={getMatches}
            disabled={!uploadComplete}
            className={`w-full py-3 px-6 rounded-xl transition-all duration-300 ${
              !uploadComplete
                ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                : "bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
            }`}
          >
            Get Matches! ❤️
          </button>
        </div>
      </div>
    </div>
  );
}
