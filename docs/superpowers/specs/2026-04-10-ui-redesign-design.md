# Docker Registry Manager UI Redesign Design

## Goals

- Rebuild the application UI into a professional control-console experience.
- Improve efficiency for the primary workflow: `Registry -> Repository -> Tag -> Manifest`.
- Keep desktop and narrow desktop windows as the primary target while preserving mobile usability.
- Unify visual language, state feedback, and destructive-action flows.

## Confirmed Product Direction

- Visual style: professional control-console.
- Workflow priority: balanced browsing and operations.
- Layout priority: desktop and narrow-window first, mobile usable.
- Theme approach: neutral professional dual-theme.
- Change scope: full information-architecture redesign is allowed.

## Current Problems

### Visual Problems

- The current three-panel layout gives every panel equal weight, so the main task area is unclear.
- Toolbar presence is weak and does not act as a real global command surface.
- Emoji-based icon buttons weaken the professional desktop-tool feel.
- Spacing, density, and typography are serviceable but not polished enough for sustained use.

### UX Problems

- Refresh, settings, delete, and state feedback are scattered across local panels.
- The user lacks a strong persistent context indicator for the active registry, repository, and tag.
- Error, loading, empty, and success states are not shown through a consistent system.
- Destructive flows are technically present but visually underpowered and not integrated into a broader action model.

### Responsive Problems

- Existing mobile behavior mostly compresses desktop panels instead of restructuring them.
- Sidebar and panel collapse styles exist in CSS, but the component structure does not meaningfully support them.

## Information Architecture

The redesigned app will use four stable zones:

1. Global top bar
2. Left resource navigation
3. Central work area
4. Right detail and action rail

This separates responsibilities cleanly:

- Left side changes scope
- Center performs the main task
- Right side exposes context details and sensitive actions
- Top bar handles global controls and app-level status

## Layout Design

### 1. Global Top Bar

The top bar becomes the fixed command and context strip.

Contents:

- Product title
- Breadcrumb-style context path
- Current view heading and supporting summary
- Global refresh action
- Theme switcher
- Settings entry
- Reserved area for transient status banners

Behavior:

- The bar remains visible while the user navigates the workspace.
- Refresh always targets the currently active scope.
- Success, warning, and error messages appear here first, then can be expanded locally if needed.

### 2. Left Resource Navigation

This rail is dedicated to registry navigation only.

Contents:

- Section title
- Add registry action
- Registry filter input
- Registry list with compact metadata

Registry item structure:

- Name
- URL preview
- Connection status indicator
- Stable action menu for edit/delete

Behavior:

- Selecting a registry updates both the center and right zones.
- Edit and delete actions remain available without depending only on hover.
- Status colors and selected styling are more explicit and consistent.

### 3. Central Work Area

This becomes the primary workspace and changes with the selected context.

State progression:

- No registry selected: guidance and empty state
- Registry selected: repository workspace
- Repository selected: tag workspace

Repository workspace contents:

- Title and count summary
- Search field
- Sort/filter controls if needed
- Refresh affordance
- Repository list

Tag workspace contents:

- Title and active repository summary
- Search field
- Batch selection tools
- Batch delete entry
- Tag list with stronger row structure

The center will not permanently show repository and tag lists side by side. Instead, it will prioritize the active level so the main task area stays focused.

### 4. Right Detail and Action Rail

The right rail is reserved for context details and sensitive actions.

When a repository is active:

- Repository summary
- Deletion entry point
- Operational hints if the repository is empty

When a tag is active:

- Manifest overview
- Digest
- Media type
- Total size
- Layer list
- Raw JSON toggle
- Detail copy helpers if added later

When nothing detailed is selected:

- Guidance on what the user can do next

This keeps technical detail visible without polluting the main list area.

## Interaction Flows

### Registry Management

- Add and edit remain modal-based for now.
- The form receives stronger hierarchy and better field grouping.
- Delete remains confirmed through a dialog.
- Registry actions move to an explicit item menu or always-visible compact controls.

### Repository Browsing

- Selecting a registry opens the repository workspace in the center.
- Search stays local to the current scope.
- Repository delete opens a structured confirmation flow in dialog form.

### Tag Browsing And Batch Operations

- Selecting a repository shifts the center workspace to tag management.
- Batch selection becomes visually clearer and pinned near the list header.
- Delete selected tags uses a stronger destructive confirmation pattern.
- Deletion progress and summary are shown through the unified status system.

### Manifest Inspection

- Manifest details stay in the right rail.
- Overview fields are promoted above raw JSON.
- Raw JSON is secondary and collapsible.
- Layer rows should feel like technical records, not loose text blocks.

## State System

The redesign standardizes five state classes:

- Empty
- Loading
- Success
- Warning
- Error

Rules:

- Global transient feedback appears in the top bar status area.
- Panel-local states appear inside the affected workspace using shared components and shared CSS patterns.
- Error recovery actions such as retry are always adjacent to the message.

## Visual System

### Style Direction

- Neutral, professional, high-legibility desktop tool styling
- Dual theme with equal care for light and dark modes
- No decorative gradients or playful iconography by default
- Strong borders, controlled shadows, and deliberate spacing

### Typography

- Clear hierarchy between page title, panel title, metadata, and technical values
- Monospace reserved for digests, commands, and machine-like identifiers

### Components

- Replace emoji-first icon controls with text-first or symbol-light controls
- Strengthen selected, hover, focus, disabled, and danger states
- Make destructive actions visually distinct without overpowering the whole UI

## Responsive Strategy

### Wide Desktop

- Three-column shell: left navigation, center workspace, right details

### Narrow Desktop

- Left rail narrows or collapses
- Right rail becomes a toggleable drawer or stacked section
- Center workspace remains primary

### Mobile

- One primary content column
- Registry navigation and detail rail move into drawers or stacked sections
- Critical actions remain reachable without relying on hover

The design goal on mobile is usable continuity, not full desktop parity.

## Component-Level Implementation Direction

- `src/components/app.rs`: replace the current equal-weight panel shell with the new application shell.
- `src/components/toolbar.rs`: evolve into a real top bar with context and global actions.
- `src/components/registry_list.rs`: reshape into a left navigation rail.
- `src/components/repository_list.rs`: convert into the main repository workspace.
- `src/components/tag_list.rs`: convert into the main tag workspace with stronger batch controls.
- `src/components/manifest_view.rs`: convert into the right-side detail rail content.
- `src/components/settings.rs`: keep as a separate settings view within the shell.
- `src/components/delete_dialog.rs`: visually align destructive dialogs with the new system.
- `assets/main.css`: reorganize around shell, navigation, workspace, detail rail, status system, and responsive breakpoints.

## Risks And Constraints

- The current code mixes layout and local behavior inside the same components, so some moderate restructuring is expected.
- The design should remain incremental enough to avoid breaking existing registry, repository, tag, and manifest fetching logic.
- Current state is mostly sufficient, so this redesign should avoid broad data-model refactors unless needed for UI composition.

## Testing Expectations

- Desktop shell renders correctly with no selection, registry selected, repository selected, and settings open.
- Narrow window layout keeps primary workflows reachable.
- Mobile layout does not hide essential actions behind hover-only behavior.
- Delete dialogs, loading indicators, and error states remain functional after the redesign.
