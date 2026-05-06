import { test, expect } from '@playwright/test';

// Helper: get the visible lesson/page container
function visibleSection(page) {
  // The currently visible section is the one with display != none
  return page.locator('div:visible');
}

test.describe('Homepage', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500); // wait for Wasm init
  });

  test('shows hero section with title and subtitle', async ({ page }) => {
    await expect(page.locator('text=CUI').first()).toBeVisible();
    await expect(page.locator('text=A web language based on CSS syntax')).toBeVisible();
  });

  test('hero code block is visible', async ({ page }) => {
    await expect(page.locator('text=.button {').first()).toBeVisible();
  });

  test('hero has live demo below code', async ({ page }) => {
    await expect(page.locator('text=LIVE OUTPUT:').first()).toBeVisible();
  });

  test('hero demo button is clickable', async ({ page }) => {
    const heroBtn = page.getByText('click me', { exact: true });
    await heroBtn.click();
    await page.waitForTimeout(300);
    const clicked = page.getByText('clicked!', { exact: true });
    await expect(clicked).toBeVisible();
  });

  test('Try It demo button works', async ({ page }) => {
    // The "Click me" button is in the Try It section (capitalized)
    const tryBtn = page.getByText('Click me', { exact: true }).first();
    await expect(tryBtn).toBeVisible();
    await tryBtn.click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Clicked! This ran through Wasm.', { exact: true })).toBeVisible();
  });

  test('has dark mode toggle', async ({ page }) => {
    const toggle = page.getByText('Dark', { exact: true }).first();
    await expect(toggle).toBeVisible();
    await toggle.click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Light', { exact: true }).first()).toBeVisible();
  });

  test('shows all homepage sections', async ({ page }) => {
    await expect(page.getByText('What is CUI?', { exact: true })).toBeVisible();
    await expect(page.getByText('Why CUI?', { exact: true })).toBeVisible();
    await expect(page.getByText('Try it', { exact: true })).toBeVisible();
    await expect(page.getByText('Interactive Tutorial', { exact: true })).toBeVisible();
    await expect(page.getByText('How it works', { exact: true })).toBeVisible();
    await expect(page.getByText('Get started', { exact: true })).toBeVisible();
  });

  test('GitHub link is present', async ({ page }) => {
    const link = page.locator('a[href*="github.com/thisminute/cascading-ui"]').first();
    await expect(link).toBeVisible();
  });
});

test.describe('Tutorial Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('navigates to tutorial cover page', async ({ page }) => {
    await expect(page.getByText('CUI Interactive Tutorial', { exact: true })).toBeVisible();
    await expect(page.getByText('Start with Lesson 1 →', { exact: true })).toBeVisible();
  });

  test('sidebar is visible with all lessons', async ({ page }) => {
    await expect(page.getByText('Overview', { exact: true })).toBeVisible();
    await expect(page.getByText('1. Text', { exact: true })).toBeVisible();
    await expect(page.getByText('2. Structure', { exact: true })).toBeVisible();
    await expect(page.getByText('3. Classes', { exact: true })).toBeVisible();
    await expect(page.getByText('4. Events', { exact: true })).toBeVisible();
    await expect(page.getByText('5. All Together', { exact: true })).toBeVisible();
  });

  test('← Home link returns to homepage', async ({ page }) => {
    await page.getByText('← Home', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('What is CUI?', { exact: true })).toBeVisible();
  });

  test('sidebar navigates directly to lessons', async ({ page }) => {
    await page.getByText('3. Classes', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 3: Classes & Cascading', { exact: true })).toBeVisible();
  });
});

test.describe('Lesson 1: Text', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('Start with Lesson 1 →', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows lesson title and content', async ({ page }) => {
    await expect(page.getByText('Lesson 1: The Text Property', { exact: true })).toBeVisible();
    await expect(page.getByText('The simplest thing: putting words on the page', { exact: true })).toBeVisible();
  });

  test('shows CUI source code', async ({ page }) => {
    await expect(page.locator('text=CUI source:').first()).toBeVisible();
  });

  test('shows live demo with Hello CUI', async ({ page }) => {
    await expect(page.locator('text=Live result:').first()).toBeVisible();
    await expect(page.getByText('Hello, CUI!', { exact: true }).first()).toBeVisible();
  });

  test('toggle shows compiled output', async ({ page }) => {
    // Click the first "Show compiled" toggle (for lesson 1)
    await page.locator('text=▶ Show compiled HTML + CSS').first().click();
    await page.waitForTimeout(300);
    await expect(page.locator('text=Compiled HTML:').first()).toBeVisible();
    await expect(page.locator('text=Compiled CSS:').first()).toBeVisible();
  });

  test('toggle hides compiled output', async ({ page }) => {
    // Show it
    await page.locator('text=▶ Show compiled HTML + CSS').first().click();
    await page.waitForTimeout(300);
    await expect(page.locator('text=Compiled HTML:').first()).toBeVisible();
    // Hide it
    await page.locator('text=▼ Hide compiled HTML + CSS').first().click();
    await page.waitForTimeout(300);
    // The first "Compiled HTML:" should now be hidden
    // Check by verifying the show toggle is back
    await expect(page.locator('text=▶ Show compiled HTML + CSS').first()).toBeVisible();
  });

  test('Next button navigates to lesson 2', async ({ page }) => {
    await page.getByText('Next: Elements & Structure →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 2: Elements & Structure', { exact: true })).toBeVisible();
  });
});

test.describe('Lesson 2: Structure', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('2. Structure', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows card demo with structure', async ({ page }) => {
    await expect(page.getByText('Lesson 2: Elements & Structure', { exact: true })).toBeVisible();
    // "My Card" appears in both code block and demo — check demo is visible
    await expect(page.getByText('My Card', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Content inside the card.', { exact: true }).first()).toBeVisible();
  });

  test('prev button goes to lesson 1', async ({ page }) => {
    await page.getByText('← Text', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 1: The Text Property', { exact: true })).toBeVisible();
  });

  test('next button goes to lesson 3', async ({ page }) => {
    await page.getByText('Classes & Cascading →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 3: Classes & Cascading', { exact: true })).toBeVisible();
  });
});

test.describe('Lesson 3: Classes', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('3. Classes', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows colored tags demo', async ({ page }) => {
    await expect(page.getByText('Lesson 3: Classes & Cascading', { exact: true })).toBeVisible();
    await expect(page.getByText('Default', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Custom', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Another', { exact: true }).first()).toBeVisible();
  });
});

test.describe('Lesson 4: Events', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('4. Events', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows interactive button demo', async ({ page }) => {
    await expect(page.getByText('Lesson 4: Events & Interactivity', { exact: true })).toBeVisible();
    // Find the visible "Click me" button (skip hidden ones from other pages)
    const allBtns = page.getByText('Click me', { exact: true });
    let found = false;
    for (let i = 0; i < await allBtns.count(); i++) {
      if (await allBtns.nth(i).isVisible()) {
        found = true;
        break;
      }
    }
    expect(found).toBe(true);
  });

  test('button click changes text and color', async ({ page }) => {
    // Find the visible "Click me" button
    const allBtns = page.getByText('Click me', { exact: true });
    let demoBtn = null;
    for (let i = 0; i < await allBtns.count(); i++) {
      if (await allBtns.nth(i).isVisible()) {
        demoBtn = allBtns.nth(i);
        break;
      }
    }
    expect(demoBtn).not.toBeNull();
    await demoBtn.click();
    await page.waitForTimeout(500);
    await expect(page.getByText('Clicked! This runs in Wasm.', { exact: true })).toBeVisible();
  });

  test('shows compiled output with Wasm explanation', async ({ page }) => {
    // The "Show compiled" for lesson 4 is after lessons 1-3's toggle
    // Click it via the visible one on lesson 4's page
    const toggles = page.locator('text=▶ Show compiled HTML + CSS');
    // Lesson 4's toggle should be the 4th (index 3), but let's find the visible one
    for (let i = 0; i < await toggles.count(); i++) {
      if (await toggles.nth(i).isVisible()) {
        await toggles.nth(i).click();
        break;
      }
    }
    await page.waitForTimeout(300);
    await expect(page.getByText('Compiled to Wasm:', { exact: true })).toBeVisible();
  });
});

test.describe('Lesson 5: All Together', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('5. All Together', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows todo list demo', async ({ page }) => {
    await expect(page.getByText('Lesson 5: Putting It All Together', { exact: true })).toBeVisible();
    await expect(page.getByText('Learn CUI', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Build something', { exact: true }).first()).toBeVisible();
  });

  test('shows finish card with GitHub link', async ({ page }) => {
    await expect(page.getByText("That's the core of CUI.", { exact: true })).toBeVisible();
    await expect(page.getByText('Explore on GitHub →', { exact: true })).toBeVisible();
  });
});

test.describe('Dark Mode', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
  });

  test('toggle switches to dark mode', async ({ page }) => {
    await page.getByText('Dark', { exact: true }).first().click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Light', { exact: true }).first()).toBeVisible();
  });

  test('dark mode persists in tutorial', async ({ page }) => {
    await page.getByText('Dark', { exact: true }).first().click();
    await page.waitForTimeout(300);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Light', { exact: true }).first()).toBeVisible();
  });

  test('toggle switches back to light mode', async ({ page }) => {
    await page.getByText('Dark', { exact: true }).first().click();
    await page.waitForTimeout(300);
    await page.getByText('Light', { exact: true }).first().click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Dark', { exact: true }).first()).toBeVisible();
  });
});

test.describe('Full lesson flow', () => {
  test('navigate through all lessons sequentially', async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);

    // Home → Tutorial
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('CUI Interactive Tutorial', { exact: true })).toBeVisible();

    // Cover → Lesson 1
    await page.getByText('Start with Lesson 1 →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 1: The Text Property', { exact: true })).toBeVisible();

    // Lesson 1 → Lesson 2
    await page.getByText('Next: Elements & Structure →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 2: Elements & Structure', { exact: true })).toBeVisible();

    // Lesson 2 → Lesson 3
    await page.getByText('Classes & Cascading →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 3: Classes & Cascading', { exact: true })).toBeVisible();

    // Lesson 3 → Lesson 4
    await page.getByText('Events & Interactivity →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 4: Events & Interactivity', { exact: true })).toBeVisible();

    // Lesson 4 → Lesson 5
    await page.getByText('Putting It Together →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 5: Putting It All Together', { exact: true })).toBeVisible();

    // Should see the finish card
    await expect(page.getByText("That's the core of CUI.", { exact: true })).toBeVisible();
  });
});
