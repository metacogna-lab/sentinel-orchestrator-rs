/**
 * Chaos view - Chaos generator for backend stress testing
 * High-frequency request generator that adheres to backend contracts
 */

import { ChaosGenerator } from '../components/chaos';

export function Chaos() {
  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-display-2 font-display text-rust-orange mb-2">
          Chaos Generator
        </h1>
        <p className="text-light-gray text-body-lg">
          High-frequency chat completion request generator for backend stress testing
        </p>
      </div>
      <ChaosGenerator />
    </div>
  );
}

