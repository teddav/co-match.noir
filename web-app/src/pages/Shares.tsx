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

  const uploadShares = async () => {
    if (!shareData) return;

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
        twitter_handle: "@HELLO",
      });

      const response = await fetch("http://localhost:8000/upload?" + params.toString(), {
        method: "POST",
        body: formData,
      });

      const data = await response.json();
      console.log(data);
      setUploadComplete(true);
    } catch (error) {
      console.error(error);
      setUploadError("Failed to upload shares. Please try again.");
    } finally {
      setIsUploading(false);
    }
  };

  const handleSecondAction = () => {
    // This will be implemented later
    console.log("Second action triggered");
  };

  return (
    <div className="bg-white rounded-lg shadow-lg p-6">
      <h2 className="text-2xl font-semibold text-gray-800 mb-6">Share Management</h2>

      <div className="space-y-6">
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
            disabled={isUploading || uploadComplete || !shareData}
            className={`w-full py-2 px-4 rounded-md transition-colors ${
              isUploading || uploadComplete || !shareData
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
              <h3 className="text-lg font-medium text-gray-900">Step 2: Process Shares</h3>
              <p className="text-sm text-gray-500">Process your shares to find matches</p>
            </div>
          </div>

          <button
            onClick={handleSecondAction}
            disabled={!uploadComplete}
            className={`w-full py-2 px-4 rounded-md transition-colors ${
              !uploadComplete ? "bg-gray-100 text-gray-400 cursor-not-allowed" : "bg-purple-600 text-white hover:bg-purple-700"
            }`}
          >
            Process Shares
          </button>
        </div>
      </div>
    </div>
  );
}
