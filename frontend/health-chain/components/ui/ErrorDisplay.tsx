"use client";

import React from "react";
import { AlertCircle, RefreshCw } from "lucide-react";
import { cn } from "@/lib/utils/cn";

interface ErrorDisplayProps {
  /** The error message to display */
  message: string;
  /** Optional heading – defaults to "Something went wrong" */
  heading?: string;
  /** If provided, renders a Retry button */
  onRetry?: () => void;
  /** Optional additional class names */
  className?: string;
}

/**
 * Shared error display card.
 * Use this as the consistent error state for all React Query-powered pages.
 *
 * @example
 * if (isError) return <ErrorDisplay message={error.message} onRetry={refetch} />;
 */
export function ErrorDisplay({
  message,
  heading = "Something went wrong",
  onRetry,
  className,
}: ErrorDisplayProps) {
  return (
    <div
      className={cn(
        "rounded-lg border border-red-200 bg-red-50 p-4",
        className,
      )}
      role="alert"
    >
      <div className="flex items-start gap-3">
        <AlertCircle
          className="mt-0.5 h-5 w-5 flex-shrink-0 text-red-600"
          aria-hidden="true"
        />
        <div className="flex-1">
          <p className="font-medium text-red-800">{heading}</p>
          <p className="mt-1 text-sm text-red-700">{message}</p>
        </div>
        {onRetry && (
          <button
            onClick={onRetry}
            className="flex items-center gap-1.5 rounded bg-red-600 px-3 py-1.5 text-sm text-white transition-colors hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-1"
          >
            <RefreshCw size={13} aria-hidden="true" />
            Retry
          </button>
        )}
      </div>
    </div>
  );
}
