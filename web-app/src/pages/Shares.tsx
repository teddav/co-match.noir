import { useState } from "react";

interface SharesProps {
  shareData: {
    shares: Uint8Array[];
  } | null;
}

export default function Shares({ shareData }: SharesProps) {
  const [isUploading, setIsUploading] = useState(false);
  const [uploadComplete, setUploadComplete] = useState(false);
  const [uploadError, setUploadError] = useState<string | null>(null);
  const [twitterHandle, setTwitterHandle] = useState("");

  const uploadShares = async () => {
    if (!shareData) return;
    if (!twitterHandle) {
      setUploadError("Please enter your Twitter handle");
      return;
    }

    setIsUploading(true);
    setUploadError(null);

    try {
      const formData = new FormData();

      // Add each share as a separate file
      shareData.shares.forEach((share, index) => {
        const blob = new Blob([share], { type: "application/octet-stream" });
        formData.append(`file${index}`, blob);
      });

      const params = new URLSearchParams({
        twitter_handle: twitterHandle,
      });

      const response = await fetch("http://localhost:8000/upload?" + params.toString(), {
        method: "POST",
        body: formData,
      });

      const data = await response.json();
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

  const getMatches = async () => {
    const token = localStorage.getItem("co-match-token");
    if (!token) {
      setUploadError("No token found. Please try uploading your shares again.");
      setUploadComplete(false);
      return;
    }

    try {
      const response = await fetch("http://localhost:8000/", {
        method: "GET",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      const data = await response.json();
      console.log("Matches:", data);
    } catch (error) {
      console.error(error);
      setUploadError("Failed to get matches. Please try again.");
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-lg p-6">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">Share Management</h2>

      <div className="space-y-6">
        {/* Twitter Handle Input */}
        <div className="border rounded-lg p-4">
          <div className="mb-4">
            <h3 className="text-lg font-medium text-gray-900 mb-2">Twitter Handle</h3>
            <p className="text-sm text-gray-600 mb-4">To allow your matches to contact you, please enter your Twitter handle.</p>
            <div className="flex items-center space-x-2">
              <span className="text-gray-500">@</span>
              <input
                type="text"
                value={twitterHandle}
                onChange={(e) => setTwitterHandle(e.target.value.replace("@", ""))}
                placeholder="your_twitter_handle"
                className="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-purple-500"
              />
            </div>
          </div>
        </div>

        {/* First Step */}
        <div className="border rounded-lg p-4">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-medium text-gray-900">Step 1: Upload Shares</h3>
              <p className="text-sm text-gray-500">Upload your preference shares to the network</p>
            </div>
            {uploadComplete && <span className="text-green-600 text-sm font-medium">✓ Complete</span>}
          </div>

          {uploadError && <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-md text-sm">{uploadError}</div>}

          <button
            onClick={uploadShares}
            disabled={isUploading || uploadComplete || !shareData || !twitterHandle}
            className={`w-full py-2 px-4 rounded-md transition-colors ${
              isUploading || uploadComplete || !shareData || !twitterHandle
                ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                : "bg-purple-600 text-white hover:bg-purple-700"
            }`}
          >
            {isUploading ? "Uploading..." : uploadComplete ? "Uploaded" : "Upload Shares"}
          </button>
        </div>

        {/* Second Step */}
        <div className="border rounded-lg p-4">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h3 className="text-lg font-medium text-gray-900">Step 2: Match!!!</h3>
              <p className="text-sm text-gray-500">Process your encrypted profile to find matches</p>
            </div>
          </div>

          <button
            onClick={getMatches}
            disabled={!uploadComplete}
            className={`w-full py-2 px-4 rounded-md transition-colors ${
              !uploadComplete ? "bg-gray-100 text-gray-400 cursor-not-allowed" : "bg-purple-600 text-white hover:bg-purple-700"
            }`}
          >
            Get Matches! ❤️
          </button>
        </div>
      </div>
    </div>
  );
}
