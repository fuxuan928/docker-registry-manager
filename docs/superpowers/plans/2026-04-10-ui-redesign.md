# UI Redesign Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rebuild the app into a professional control-console layout with a stronger workflow for registry, repository, tag, and manifest management.

**Architecture:** Keep the existing data-fetching and state model where possible, but replace the current equal-weight three-panel layout with a shell composed of a top bar, left navigation, central workspace, and right detail rail. The implementation should favor small structural changes inside existing components instead of introducing unnecessary new state layers.

**Tech Stack:** Rust, Dioxus 0.7, CSS

---

## File Structure

- Modify: `src/components/app.rs`
  Responsibility: define the new shell and route visible regions based on selection state.
- Modify: `src/components/toolbar.rs`
  Responsibility: render top bar context, actions, and status area.
- Modify: `src/components/registry_list.rs`
  Responsibility: present the left navigation rail and registry actions.
- Modify: `src/components/repository_list.rs`
  Responsibility: render the repository workspace.
- Modify: `src/components/tag_list.rs`
  Responsibility: render the tag workspace and batch actions.
- Modify: `src/components/manifest_view.rs`
  Responsibility: render the detail rail content.
- Modify: `src/components/delete_dialog.rs`
  Responsibility: align destructive dialogs with the new design language.
- Modify: `src/components/settings.rs`
  Responsibility: align settings view with the new shell.
- Modify: `assets/main.css`
  Responsibility: implement the entire visual system and responsive layout.

### Task 1: Rebuild The Application Shell

**Files:**
- Modify: `src/components/app.rs`
- Modify: `src/components/toolbar.rs`
- Test: manual verification via `cargo check`

- [ ] **Step 1: Write the failing test**

There is no UI test harness in the repository today, so the first failure signal is a compile-safe shell refactor target. Define the expected component responsibilities before changing code:

```rust
// Desired shell responsibilities after refactor:
// - Toolbar renders global context and actions.
// - App renders left navigation, center workspace, and right detail rail.
// - Settings replaces the workspace content when open.
```

- [ ] **Step 2: Run the current compile check as the baseline**

Run: `cargo check`
Expected: PASS before the shell refactor begins.

- [ ] **Step 3: Write minimal shell implementation**

Update `src/components/app.rs` to:

```rust
rsx! {
    div {
        class: "app-shell",
        "data-theme": theme_class,

        Toolbar { show_settings }

        div {
            class: "app-body",

            aside {
                class: "left-rail",
                RegistryList {}
            }

            main {
                class: "workspace",
                if show_settings() {
                    Settings {}
                } else {
                    RepositoryList {}
                    TagList {}
                }
            }

            aside {
                class: "detail-rail",
                if !show_settings() {
                    ManifestView {}
                }
            }
        }
    }
}
```

Update `src/components/toolbar.rs` to include:

```rust
let registry = app_state.selected_registry.read().clone();
let repo = app_state.selected_repo.read().clone();
let tag = app_state.selected_tag.read().clone();

let context_label = match (registry.as_ref(), repo.as_ref(), tag.as_ref()) {
    (_, _, Some(tag)) => format!("Tag / {tag}"),
    (_, Some(repo), None) => format!("Repository / {repo}"),
    (Some(_), None, None) => "Registry selected".to_string(),
    _ => "No selection".to_string(),
};
```

- [ ] **Step 4: Run compile check after shell changes**

Run: `cargo check`
Expected: PASS with the new shell structure in place.

### Task 2: Convert Registry List Into A Left Navigation Rail

**Files:**
- Modify: `src/components/registry_list.rs`
- Test: manual verification via `cargo check`

- [ ] **Step 1: Write the failing test**

Document the interaction requirements before editing:

```rust
// Desired registry rail behavior:
// - Dedicated search field
// - Stable add action in the rail header
// - Registry rows show metadata and controls without hover-only dependency
```

- [ ] **Step 2: Run compile check before editing the component**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 3: Write minimal registry rail implementation**

Refactor the component structure around a rail header, filter input, and list body:

```rust
div {
    class: "rail-section",
    div {
        class: "rail-header",
        div {
            class: "rail-title-group",
            h2 { "Registries" }
            p { "Manage available registry connections" }
        }
        button { class: "primary small", "+ Add" }
    }
    div {
        class: "rail-search",
        input { placeholder: "Filter registries..." }
    }
    div { class: "rail-list", /* registry items */ }
}
```

Update each registry item so it has visible metadata and action controls:

```rust
div {
    class: if is_selected { "registry-card selected" } else { "registry-card" },
    div { class: "registry-card-main", /* name and url */ }
    div { class: "registry-card-meta", /* status text */ }
    div { class: "registry-card-actions", /* edit/delete */ }
}
```

- [ ] **Step 4: Run compile check after registry rail refactor**

Run: `cargo check`
Expected: PASS.

### Task 3: Convert Repository And Tag Views Into The Main Workspace

**Files:**
- Modify: `src/components/repository_list.rs`
- Modify: `src/components/tag_list.rs`
- Test: manual verification via `cargo check`

- [ ] **Step 1: Write the failing test**

Define the target workspace behavior:

```rust
// Desired workspace behavior:
// - Repository view is primary when only a registry is selected.
// - Tag view is primary when a repository is selected.
// - Each view owns its own heading, search field, and local actions.
```

- [ ] **Step 2: Run compile check before workspace changes**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 3: Write minimal repository workspace implementation**

Wrap repository content in a workspace section:

```rust
div {
    class: if selected_repo.is_some() { "workspace-panel is-secondary" } else { "workspace-panel is-primary" },
    div {
        class: "workspace-header",
        div {
            h2 { "Repositories" }
            p { "Browse repositories in the selected registry" }
        }
        button { class: "secondary small", "Refresh" }
    }
    // existing search, status, and list content
}
```

- [ ] **Step 4: Write minimal tag workspace implementation**

Wrap tag content similarly and promote batch actions near the header:

```rust
div {
    class: if selected_repo.is_some() { "workspace-panel is-primary" } else { "workspace-panel is-secondary" },
    div {
        class: "workspace-header",
        div {
            h2 { "Tags" }
            p { "Inspect and manage image tags" }
        }
        div { class: "workspace-actions", /* refresh and batch actions */ }
    }
    // existing search, delete status, and list content
}
```

- [ ] **Step 5: Run compile check after workspace changes**

Run: `cargo check`
Expected: PASS.

### Task 4: Move Manifest Into A Dedicated Detail Rail

**Files:**
- Modify: `src/components/manifest_view.rs`
- Test: manual verification via `cargo check`

- [ ] **Step 1: Write the failing test**

Define the detail-rail expectations:

```rust
// Desired detail rail behavior:
// - Show guidance when no tag is selected.
// - Show manifest overview as technical detail content when a tag is selected.
// - Keep raw JSON secondary and collapsible.
```

- [ ] **Step 2: Run compile check before editing the detail view**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 3: Write minimal detail rail implementation**

Wrap manifest content in a right-rail card structure:

```rust
div {
    class: "detail-panel",
    div {
        class: "detail-header",
        h2 { "Details" }
        p { "Manifest and technical metadata" }
    }
    // existing loading, error, empty, and manifest states
}
```

Promote the overview block and keep raw JSON behind the toggle button.

- [ ] **Step 4: Run compile check after detail rail refactor**

Run: `cargo check`
Expected: PASS.

### Task 5: Redesign Shared Status, Dialog, And Settings Surfaces

**Files:**
- Modify: `src/components/delete_dialog.rs`
- Modify: `src/components/settings.rs`
- Modify: `src/components/toolbar.rs`
- Test: manual verification via `cargo check`

- [ ] **Step 1: Write the failing test**

Define the surface expectations:

```rust
// Desired supporting-surface behavior:
// - Delete dialogs match the new shell styling.
// - Settings reads as a first-class page in the shell.
// - Toolbar can host global status feedback.
```

- [ ] **Step 2: Run compile check before supporting-surface changes**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 3: Write minimal supporting-surface implementation**

Update settings content wrappers:

```rust
div {
    class: "settings-page",
    div {
        class: "settings-page-header",
        h1 { "Settings" }
        p { "Appearance, cache, and configuration transfer" }
    }
    // existing settings sections
}
```

Update delete dialogs to use stronger card sections and summary blocks. Reserve a toolbar sub-area for status banners:

```rust
div {
    class: "toolbar-status-slot",
    span { "Ready" }
}
```

- [ ] **Step 4: Run compile check after supporting-surface refactor**

Run: `cargo check`
Expected: PASS.

### Task 6: Rebuild The CSS System Around The New Shell

**Files:**
- Modify: `assets/main.css`
- Test: manual verification via `cargo check`

- [ ] **Step 1: Write the failing test**

List the CSS responsibilities to satisfy:

```css
/* Desired stylesheet responsibilities:
   - shell layout
   - navigation rail
   - workspace panel styles
   - detail rail
   - unified statuses
   - dialogs
   - responsive collapse for narrow windows and mobile
*/
```

- [ ] **Step 2: Run compile check before stylesheet rewrite**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 3: Write minimal shell-oriented CSS implementation**

Add or replace the core layout classes:

```css
.app-shell {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
    color: var(--text-primary);
}

.app-body {
    display: grid;
    grid-template-columns: 320px minmax(0, 1fr) 360px;
    gap: 0;
    min-height: 0;
    flex: 1;
}

.left-rail,
.workspace,
.detail-rail {
    min-width: 0;
    min-height: 0;
}
```

Add matching class families for:

```css
.topbar {}
.rail-section {}
.registry-card {}
.workspace-panel {}
.detail-panel {}
.status-banner {}
.settings-page {}
```

Then rewrite responsive breakpoints so narrow windows stack the detail rail below the workspace and mobile collapses into a single-column flow.

- [ ] **Step 4: Run compile check after stylesheet rewrite**

Run: `cargo check`
Expected: PASS.

### Task 7: Verify The End-To-End Redesign

**Files:**
- Modify: no new code if verification passes
- Test: manual verification commands and visual checks

- [ ] **Step 1: Run final compile verification**

Run: `cargo check`
Expected: PASS.

- [ ] **Step 2: Run the app for visual verification**

Run: `dx serve`
Expected: the shell renders with the new layout and no runtime panic.

- [ ] **Step 3: Verify the core scenarios manually**

Check:

```text
1. No registry selected: guidance state is visible.
2. Registry selected: repository workspace is primary.
3. Repository selected: tag workspace is primary.
4. Tag selected: manifest detail rail shows overview data.
5. Settings open: shell swaps to the settings page cleanly.
6. Narrow window: right detail rail reflows without hiding core actions.
7. Mobile width: app remains usable without hover-only controls.
```

- [ ] **Step 4: Address any verification failures**

If `cargo check` fails, fix the named file and rerun `cargo check` until it passes. If the app layout is broken in `dx serve`, adjust the corresponding shell or CSS class and re-run `dx serve`.
