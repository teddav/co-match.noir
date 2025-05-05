import { useState, useEffect } from "react";
import Preferences, { UserPreferences } from "./Preferences";
import Shares from "./Shares";
import Matches from "./Matches";

type View = "preferences" | "shares" | "matches";

const CIRCUIT_VERSION = 1;

export default function Home() {
  const [view, setView] = useState<View>("preferences");
  const [preferences, setPreferences] = useState<UserPreferences | null>(null);
  const [showPreferences, setShowPreferences] = useState(false);

  useEffect(() => {
    const storedVersion = localStorage.getItem("co-match-circuit-version");
    if (storedVersion !== String(CIRCUIT_VERSION)) {
      // Version changed or not set, reset everything
      localStorage.clear();
      localStorage.setItem("co-match-circuit-version", String(CIRCUIT_VERSION));
      setView("preferences");
      setPreferences(null);
      setShowPreferences(false);
      window.location.reload();
      return;
    }
  }, []);

  useEffect(() => {
    const savedPreferences = localStorage.getItem("co-match-preferences");
    const token = localStorage.getItem("co-match-token");

    if (savedPreferences && token) {
      setView("matches");
      setPreferences(JSON.parse(savedPreferences));
    } else if (savedPreferences) {
      setView("shares");
      setPreferences(JSON.parse(savedPreferences));
    }
  }, []);

  const handlePreferencesSubmit = (newPreferences: UserPreferences) => {
    setPreferences(newPreferences);
    setView("shares");
    setShowPreferences(false);
  };

  const handleMatchesSubmit = () => {
    setView("matches");
  };

  const renderContent = () => {
    switch (view) {
      case "preferences":
        return <Preferences onSubmit={handlePreferencesSubmit} />;
      case "shares":
        return preferences ? <Shares preferences={preferences} getMatches={handleMatchesSubmit} /> : null;
      case "matches":
        return <Matches />;
      default:
        return <Preferences onSubmit={handlePreferencesSubmit} />;
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-pink-50 to-purple-50">
      <div className="max-w-4xl mx-auto px-4 py-8">
        <div className="flex justify-between items-center mb-4">
          <h1 className="text-4xl font-bold text-purple-800">Co-Match Dating</h1>
        </div>
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

        {view !== "preferences" && (
          <button
            onClick={() => setShowPreferences(!showPreferences)}
            className="px-4 py-2 bg-purple-100 text-purple-700 rounded-md hover:bg-purple-200 transition-colors"
          >
            {showPreferences ? "Hide Preferences" : "View Preferences"}
          </button>
        )}
        <div className="grid gap-8">
          {showPreferences ? (
            <div className="space-y-8">
              <Preferences onSubmit={handlePreferencesSubmit} />
            </div>
          ) : (
            renderContent()
          )}
        </div>
      </div>
    </div>
  );
}
