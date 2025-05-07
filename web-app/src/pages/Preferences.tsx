import { useState, useEffect } from "react";

export interface UserPreferences {
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

interface PreferencesProps {
  onSubmit: (preferences: UserPreferences) => void;
}

const INTERESTS = [
  { id: 1, label: "Zero-Knowledge Proofs 🕵️‍♂️" },
  { id: 2, label: "Multi-Party Computation 👫" },
  { id: 3, label: "Fully Homomorphic Encryption 🔐" },
  { id: 4, label: "Noir 🖤" },
  { id: 5, label: "I'm not fun... 😏" },
];

const REGIONS = [
  { id: 1, label: "Europe" },
  { id: 2, label: "Americas" },
  { id: 3, label: "Asia" },
  { id: 4, label: "Africa" },
];

export default function Preferences({ onSubmit }: PreferencesProps) {
  const [preferences, setPreferences] = useState<UserPreferences>({
    id: "0x" + Math.random().toString(16).substring(2, 15),
    age: 25,
    gender: 0,
    interests: [],
    region: 1,
    preferences: {
      age_min: 20,
      age_max: 80,
      gender: 2,
    },
  });

  const [isReadOnly, setIsReadOnly] = useState(false);

  useEffect(() => {
    const savedPreferences = localStorage.getItem("co-match-preferences");
    if (savedPreferences) {
      setPreferences(JSON.parse(savedPreferences));
      setIsReadOnly(true);
    }
  }, []);

  const handleInterestChange = (interestId: number) => {
    if (isReadOnly) return;
    setPreferences((prev) => ({
      ...prev,
      interests: prev.interests.includes(interestId) ? prev.interests.filter((id) => id !== interestId) : [...prev.interests, interestId],
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (isReadOnly) return;

    // Save preferences to localStorage
    localStorage.setItem("co-match-preferences", JSON.stringify(preferences));
    onSubmit(preferences);
  };

  return (
    <div className="bg-white/90 backdrop-blur-md rounded-2xl shadow-lg p-8 border border-pink-100">
      <h2 className="text-3xl font-bold bg-gradient-to-r from-pink-600 to-purple-600 bg-clip-text text-transparent mb-6">
        {isReadOnly ? "Your Saved Preferences" : "Your Preferences"}
      </h2>
      {isReadOnly && (
        <div className="mb-6 p-4 bg-gradient-to-r from-pink-50 to-purple-50 text-purple-700 rounded-xl border border-pink-100">
          Your preferences have been saved. You can view them below, but they cannot be modified.
        </div>
      )}
      <form onSubmit={handleSubmit} className="space-y-8">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Your Age</label>
            <input
              type="number"
              value={preferences.age}
              onChange={(e) => !isReadOnly && setPreferences({ ...preferences, age: parseInt(e.target.value) })}
              className={`w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all ${
                isReadOnly ? "bg-gray-50" : "hover:border-purple-300"
              }`}
              min="18"
              max="100"
              disabled={isReadOnly}
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Your Gender</label>
            <select
              value={preferences.gender}
              onChange={(e) => !isReadOnly && setPreferences({ ...preferences, gender: parseInt(e.target.value) })}
              className={`w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all ${
                isReadOnly ? "bg-gray-50" : "hover:border-purple-300"
              }`}
              disabled={isReadOnly}
            >
              <option value={0}>ZK Researcher 🤓</option>
              <option value={1}>Security Researcher 🥸</option>
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Your Region</label>
            <select
              value={preferences.region}
              onChange={(e) => !isReadOnly && setPreferences({ ...preferences, region: parseInt(e.target.value) })}
              className={`w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all ${
                isReadOnly ? "bg-gray-50" : "hover:border-purple-300"
              }`}
              disabled={isReadOnly}
            >
              {REGIONS.map((region) => (
                <option key={region.id} value={region.id}>
                  {region.label}
                </option>
              ))}
            </select>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Preferred Age Range</label>
            <div className="flex gap-4">
              <input
                type="number"
                value={preferences.preferences.age_min}
                onChange={(e) =>
                  !isReadOnly &&
                  setPreferences({
                    ...preferences,
                    preferences: { ...preferences.preferences, age_min: parseInt(e.target.value) },
                  })
                }
                className={`w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all ${
                  isReadOnly ? "bg-gray-50" : "hover:border-purple-300"
                }`}
                min="18"
                max="100"
                placeholder="Min"
                disabled={isReadOnly}
              />
              <input
                type="number"
                value={preferences.preferences.age_max}
                onChange={(e) =>
                  !isReadOnly &&
                  setPreferences({
                    ...preferences,
                    preferences: { ...preferences.preferences, age_max: parseInt(e.target.value) },
                  })
                }
                className={`w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all ${
                  isReadOnly ? "bg-gray-50" : "hover:border-purple-300"
                }`}
                min="18"
                max="100"
                placeholder="Max"
                disabled={isReadOnly}
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">Preferred Gender</label>
            <select
              value={preferences.preferences.gender}
              onChange={(e) =>
                !isReadOnly &&
                setPreferences({
                  ...preferences,
                  preferences: { ...preferences.preferences, gender: parseInt(e.target.value) },
                })
              }
              className={`w-full px-4 py-3 border border-gray-200 rounded-xl focus:ring-2 focus:ring-purple-500 focus:border-transparent transition-all ${
                isReadOnly ? "bg-gray-50" : "hover:border-purple-300"
              }`}
              disabled={isReadOnly}
            >
              <option value={0}>ZK Researcher 🤓</option>
              <option value={1}>Security Researcher 🥸</option>
              <option value={2}>I love everyone 🍑</option>
            </select>
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-3">
            Your Interests (Select up to 3)
            <span className="ml-2 text-sm text-gray-500">{preferences.interests.length}/3 selected</span>
          </label>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
            {INTERESTS.map((interest) => (
              <label
                key={interest.id}
                className={`flex items-center space-x-2 p-3 border rounded-xl cursor-pointer transition-all ${
                  isReadOnly
                    ? "bg-gray-50"
                    : !preferences.interests.includes(interest.id) && preferences.interests.length >= 3
                    ? "opacity-50 cursor-not-allowed"
                    : "hover:bg-purple-50 hover:border-purple-300"
                }`}
              >
                <input
                  type="checkbox"
                  checked={preferences.interests.includes(interest.id)}
                  onChange={() => handleInterestChange(interest.id)}
                  disabled={isReadOnly || (!preferences.interests.includes(interest.id) && preferences.interests.length >= 3)}
                  className="rounded text-purple-600 focus:ring-purple-500"
                />
                <span className="text-sm text-gray-700">{interest.label}</span>
              </label>
            ))}
          </div>
        </div>

        {!isReadOnly && (
          <button
            type="submit"
            disabled={preferences.interests.length !== 3}
            className={`w-full py-3 px-6 rounded-xl transition-all duration-300 ${
              preferences.interests.length !== 3
                ? "bg-gray-100 text-gray-400 cursor-not-allowed"
                : "bg-gradient-to-r from-pink-500 to-purple-500 text-white hover:from-pink-600 hover:to-purple-600 shadow-md hover:shadow-lg transform hover:-translate-y-0.5"
            }`}
          >
            {preferences.interests.length === 3
              ? "Save Preferences"
              : `Select ${3 - preferences.interests.length} more interest${3 - preferences.interests.length === 1 ? "" : "s"}`}
          </button>
        )}
      </form>
    </div>
  );
}
