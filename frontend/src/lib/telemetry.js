// Telemetry & Event logging helper for CarbonX

let listeners = [];
let eventLogs = [];

export function subscribeToTelemetry(callback) {
  listeners.push(callback);
  // Send current logs to new listener
  callback(eventLogs);
  return () => {
    listeners = listeners.filter(l => l !== callback);
  };
}

export function logEvent(category, action, label = "", metadata = {}) {
  const timestamp = new Date().toLocaleTimeString();
  const logEntry = {
    timestamp,
    category,
    action,
    label,
    metadata,
    id: Math.random().toString(36).substr(2, 9)
  };
  
  // Keep logs capped at 100 entries
  eventLogs = [logEntry, ...eventLogs].slice(0, 100);
  
  // Notify subscribers
  listeners.forEach(listener => listener(eventLogs));
  
  // Console logging for developer inspection
  console.log(`[Telemetry] [${timestamp}] [${category.toUpperCase()}] ${action} ${label ? `(${label})` : ""}`, metadata);
}

// Function to track RPC or API call latency
export async function trackLatency(name, apiCallPromise) {
  const start = performance.now();
  try {
    const result = await apiCallPromise;
    const end = performance.now();
    const durationMs = Math.round(end - start);
    logEvent("performance", "api_call_success", name, { durationMs });
    return result;
  } catch (error) {
    const end = performance.now();
    const durationMs = Math.round(end - start);
    logEvent("performance", "api_call_failure", name, { durationMs, error: error.message || error });
    throw error;
  }
}
