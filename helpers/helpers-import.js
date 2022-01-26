import React from "react";

export function create_fragment(...args) {
  React.createElement(React.Fragment, ...args);
}
