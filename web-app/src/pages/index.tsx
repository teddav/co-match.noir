import { useState } from "react";
import Preferences from "./Preferences";
import Shares from "./Shares";

interface ShareData {
  shares: Uint8Array[];
  // Add other response fields as needed
}

export default function Home() {
  const [showPreferences, setShowPreferences] = useState(true);
  const [shareData, setShareData] = useState<ShareData | null>(null);

  const handlePreferencesSubmit = (data: ShareData) => {
    setShareData(data);
    setShowPreferences(false);
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-pink-50 to-purple-50">
      <div className="max-w-4xl mx-auto px-4 py-8">
        <h1 className="text-4xl font-bold text-center text-purple-800 mb-4">Co-Match Dating</h1>
        <div className="bg-white/80 backdrop-blur-sm rounded-lg p-6 mb-8 shadow-sm">
          <h2 className="text-xl font-semibold text-purple-700 mb-3">How It Works</h2>
          <p className="text-gray-700 mb-4">
            Co-Match is a privacy-focused dating app that uses <b>zero-knowledge proofs</b> and <b>multiparty computation</b> to match you
            with potential partners without revealing your personal information. Here&apos;s how it works:
          </p>
          <ol className="list-decimal list-inside space-y-2 text-gray-600">
            <li>Set your profile and preferences (you can select up to 3 interests)</li>
            <li>Our system will find potential matches based on your criteria</li>
            <li>If there&apos;s a mutual match, you&apos;ll both be notified</li>
            <li>Your personal data remains private throughout the entire process</li>
          </ol>
        </div>
        <div className="grid gap-8">
          {showPreferences ? <Preferences onSubmit={handlePreferencesSubmit} /> : <Shares shareData={shareData} />}
        </div>
      </div>
    </div>
  );
}
