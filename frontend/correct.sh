#!/bin/bash

# This script aims to fix common Rust/Yew errors in the 'frontend' part of the Lightdash project.
# It addresses "cannot find value", "cannot find type", "trait bound not satisfied", and "unknown field" errors.
# It also removes unused imports.

set -e  # Exit immediately if a command exits with a non-zero status.
set -u # Detect uninitialized variables.

FRONTEND_PATH="./frontend"
SRC_PATH="$FRONTEND_PATH/src"
PAGES_PATH="$SRC_PATH/pages"
COMPONENTS_PATH="$SRC_PATH/components"
TYPES_PATH="$SRC_PATH/types"

# 1. Fix "cannot find value `recs` in this scope" in `yields.rs`
echo "Fixing 'cannot find value \`recs\`' error in yields.rs..."
YIELDS_RS="$PAGES_PATH/yields.rs"
if grep -q "recs.iter().map(|rec|" "$YIELDS_RS"; then
  sed -i 's/recs.iter().map(|rec|/stats.recs.iter().map(|rec|/' "$YIELDS_RS" #replacing recs with stats.recs
  echo "  - Fixed \`recs\` value error in $YIELDS_RS"
else
  echo "  - \`recs\` value not found in $YIELDS_RS, skipping fix."
fi


# 2. Fix "cannot find type `TransactionsPageComponent` in this scope" in `mod.rs`
echo "Fixing 'cannot find type \`TransactionsPageComponent\`' error in mod.rs..."
MOD_RS="$PAGES_PATH/mod.rs"
if grep -q "Page::Transactions => html! { <TransactionsPageComponent /> }," "$MOD_RS"; then
  sed -i 's/Page::Transactions => html! { <TransactionsPageComponent \/> },/Page::Transactions => html! { <ActionsPageComponent /> },/' "$MOD_RS" # Changed TransactionsPageComponent to ActionsPageComponent
  echo "  - Fixed \`TransactionsPageComponent\` type error in $MOD_RS"
else
  echo "  - \`TransactionsPageComponent\` type not found in $MOD_RS, skipping fix."
fi

# 3. Fix "cannot find type ... in module `pages`" in `lib.rs`
echo "Fixing 'cannot find type ... in module `pages`' errors in lib.rs..."
LIB_RS="$SRC_PATH/lib.rs"
PAGE_TYPES=(
    "DashboardPage"
    "ChannelsPage"
    "ActionsPage"
    "RecommendationsPage"
    "YieldsPage"
    "AlbyPage"
)

for PAGE_TYPE in "${PAGE_TYPES[@]}"; do
  if grep -q "pages::$PAGE_TYPE" "$LIB_RS"; then
    sed -i "s/pages::$PAGE_TYPE/$PAGE_TYPE/g" "$LIB_RS" # removed pages::
    echo "  - Fixed type error for $PAGE_TYPE in $LIB_RS"
  else
    echo "  - Type $PAGE_TYPE not found in $LIB_RS, skipping fix."
  fi
done



# 4. Remove unused imports
echo "Removing unused imports..."
UNUSED_IMPORTS=(
    "yew_router::prelude::*"
    "crate::Route"
    "window"
    "Object"
    "Reflect"
    "crate::components::Navbar"
    "crate::types::Route"
    "Chart"
    "Channel"
    "fetch_channels"
    "wasm_bindgen_futures::spawn_local"
    "Navbar"
    "ChannelStatus"
    "NodeStats"
    "fetch_node_stats"
    "ChartComponent as Chart"
    "NavbarComponent as Navbar"
    "OutboundLiquidityValue"
    "SuggestedFees"
    "fetch_outbound_liquidity_value"
    "fetch_suggested_fees"
    "Button"
    "Card"
)

for UNUSED_IMPORT in "${UNUSED_IMPORTS[@]}"; do
  find "$SRC_PATH" -name "*.rs" -print0 | xargs -0 -I{} sed -i "/use $UNUSED_IMPORT;/d" {} #deletes the line
  echo "  - Removed unused import: $UNUSED_IMPORT"
done

# 5. Fix "no method named `clone` found for opaque type" in `components/mod.rs`
echo "Fixing 'no method named \`clone\`' error in components/mod.rs..."
COMPONENTS_MOD_RS="$COMPONENTS_PATH/mod.rs"
if grep -q ".clone()" "$COMPONENTS_MOD_RS"; then
  sed -i 's/.clone()//g' "$COMPONENTS_MOD_RS" # remove .clone()
  echo "  - Fixed \`clone\` method error in $COMPONENTS_MOD_RS"
else
  echo "  - \`clone\` method not found in $COMPONENTS_MOD_RS, skipping fix."
fi

# 6. Fix trait bound `AssertAllProps: HasProp` not satisfied in `pages/dashboard.rs`
echo "Fixing trait bound error in pages/dashboard.rs..."
DASHBOARD_RS="$PAGES_PATH/dashboard.rs"
# This is a complex error, and the following may or may not fully resolve it.
# It's related to how properties are handled in Yew, and may require deeper code changes.
#  The following attempts to address it by adding Clone to the CardProps
if grep -q "#[derive(Properties, PartialEq, Clone)]" "$COMPONENTS_MOD_RS"; then
  echo "  - The  \`#[derive(Properties, PartialEq, Clone)]\`  is already in place."
else
   sed -i 's/#[derive(Properties, PartialEq)]/#[derive(Properties, PartialEq, Clone)]/g' "$COMPONENTS_MOD_RS"
  echo "  - Added \`Clone\` to CardProps in $COMPONENTS_MOD_RS.  You may need to manually verify the fix."
fi

# 7. Fix "no field ... on type `&NodeStats`" in `yields.rs`
echo "Fixing 'no field ... on type `&NodeStats`' error in yields.rs..."
if grep -q "stats.yield_30d" "$YIELDS_RS"; then
  sed -i 's/stats.yield_30d/stats.thirty_day_yield/g' "$YIELDS_RS" #replacing yield_30d with thirty_day_yield
  echo "  - Fixed \`yield_30d\` field error in $YIELDS_RS"
else
  echo "  - \`yield_30d\` field not found in $YIELDS_RS, skipping fix."
fi

if grep -q "stats.capacity" "$YIELDS_RS"; then
  sed -i 's/stats.capacity/stats.total_capacity/g' "$YIELDS_RS" #replacing capacity with total_capacity
  echo "  - Fixed \`capacity\` field error in $YIELDS_RS"
else
  echo "  - \`capacity\` field not found in $YIELDS_RS, skipping fix."
fi

# 8.  Fix "unused variable `on_execute_action`" in `actions.rs`
echo "Fixing unused variable error in actions.rs..."
ACTIONS_RS="$PAGES_PATH/actions.rs"
if grep -q "on_execute_action =" "$ACTIONS_RS"; then
  sed -i 's/on_execute_action = /\_on_execute_action = /g' "$ACTIONS_RS" #prefixing the variable name with _
  echo "  - Prefixed unused variable \`on_execute_action\` with _ in $ACTIONS_RS"
else
  echo "  - Variable \`on_execute_action\` not found in $ACTIONS_RS, skipping fix."
fi

echo "All fixes applied.  Please run 'cargo check' or 'cargo build' to verify."
