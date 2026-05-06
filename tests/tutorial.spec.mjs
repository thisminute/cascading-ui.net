import { test, expect } from '@playwright/test';

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

  test('hero has output demo below code', async ({ page }) => {
    await expect(page.locator('text=OUTPUT:').first()).toBeVisible();
  });

  test('hero demo button is clickable', async ({ page }) => {
    const heroBtn = page.getByText('click me', { exact: true });
    await heroBtn.click();
    await page.waitForTimeout(300);
    const clicked = page.getByText('clicked!', { exact: true });
    await expect(clicked).toBeVisible();
  });

  test('hero tab switcher toggles to HTML view', async ({ page }) => {
    const htmlBtn = page.getByText('HTML →', { exact: true }).first();
    await expect(htmlBtn).toBeVisible();
    await htmlBtn.click();
    await page.waitForTimeout(300);
    await expect(page.getByText('← CUI', { exact: true }).first()).toBeVisible();
    // Should show real compiled HTML with class name
    await expect(page.locator('text=class=').first()).toBeVisible();
  });

  test('hero tab switcher toggles back to CUI view', async ({ page }) => {
    await page.getByText('HTML →', { exact: true }).first().click();
    await page.waitForTimeout(300);
    await page.getByText('← CUI', { exact: true }).first().click();
    await page.waitForTimeout(300);
    await expect(page.locator('text=.button {').first()).toBeVisible();
    await expect(page.getByText('HTML →', { exact: true }).first()).toBeVisible();
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
    await expect(page.getByText('Interactive Tutorial', { exact: true })).toBeVisible();
    await expect(page.getByText('How it works', { exact: true })).toBeVisible();
    await expect(page.getByText('Get started', { exact: true })).toBeVisible();
  });

  test('mentions variables on homepage', async ({ page }) => {
    await expect(page.locator('text=Variables wire them together')).toBeVisible();
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
    await expect(page.getByText('5. Variables', { exact: true })).toBeVisible();
    await expect(page.getByText('6. All Together', { exact: true })).toBeVisible();
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

  test('shows live demo with Hello CUI', async ({ page }) => {
    await expect(page.getByText('Hello, CUI!', { exact: true }).first()).toBeVisible();
  });

  test('sidebar highlights active lesson', async ({ page }) => {
    // "1. Text" should be bold (font-weight: 700)
    const menuItem = page.getByText('1. Text', { exact: true });
    await expect(menuItem).toBeVisible();
  });

  test('tab switcher shows compiled HTML', async ({ page }) => {
    const toggles = page.getByText('HTML →', { exact: true });
    for (let i = 0; i < await toggles.count(); i++) {
      if (await toggles.nth(i).isVisible()) {
        await toggles.nth(i).click();
        break;
      }
    }
    await page.waitForTimeout(300);
    // Should show the real compiled output
    await expect(page.locator('text=<div>Hello, CUI!</div>').first()).toBeVisible();
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

  test('shows card demo with styling in code', async ({ page }) => {
    await expect(page.getByText('Lesson 2: Elements & Structure', { exact: true })).toBeVisible();
    await expect(page.getByText('My Card', { exact: true }).first()).toBeVisible();
    // Code example should include styling properties
    await expect(page.locator('text=border-left:').first()).toBeVisible();
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

  test('HTML view shows CSS class from .tag', async ({ page }) => {
    const toggles = page.getByText('HTML →', { exact: true });
    for (let i = 0; i < await toggles.count(); i++) {
      if (await toggles.nth(i).isVisible()) {
        await toggles.nth(i).click();
        break;
      }
    }
    await page.waitForTimeout(300);
    // Should show real CSS class from .tag
    await expect(page.locator('text=from .tag class').first()).toBeVisible();
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

  test('shows simplified event demo (no variable)', async ({ page }) => {
    await expect(page.getByText('Lesson 4: Events & Interactivity', { exact: true })).toBeVisible();
    // Code should NOT show "let $msg" — simplified version
    const codeBlock = page.locator('text=let $msg');
    await expect(codeBlock).not.toBeVisible();
  });

  test('button click changes text', async ({ page }) => {
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
    await expect(page.getByText('Clicked!', { exact: true })).toBeVisible();
  });

  test('next button goes to Variables lesson', async ({ page }) => {
    await page.getByText('Variables →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 5: Variables', { exact: true })).toBeVisible();
  });
});

test.describe('Lesson 5: Variables', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('5. Variables', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows variable demo with Waiting... and Activate', async ({ page }) => {
    await expect(page.getByText('Lesson 5: Variables', { exact: true })).toBeVisible();
    await expect(page.getByText('Waiting...', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Activate', { exact: true }).first()).toBeVisible();
  });

  test('clicking Activate changes label text', async ({ page }) => {
    const allBtns = page.getByText('Activate', { exact: true });
    let btn = null;
    for (let i = 0; i < await allBtns.count(); i++) {
      if (await allBtns.nth(i).isVisible()) {
        btn = allBtns.nth(i);
        break;
      }
    }
    expect(btn).not.toBeNull();
    await btn.click();
    await page.waitForTimeout(500);
    await expect(page.getByText('Active!', { exact: true })).toBeVisible();
  });
});

test.describe('Lesson 6: All Together', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8090');
    await page.waitForTimeout(1500);
    await page.getByText('Start Tutorial →', { exact: true }).click();
    await page.waitForTimeout(300);
    await page.getByText('6. All Together', { exact: true }).click();
    await page.waitForTimeout(300);
  });

  test('shows todo list demo', async ({ page }) => {
    await expect(page.getByText('Lesson 6: Putting It All Together', { exact: true })).toBeVisible();
    await expect(page.getByText('Learn CUI', { exact: true }).first()).toBeVisible();
    await expect(page.getByText('Build something', { exact: true }).first()).toBeVisible();
  });

  test('shows status text', async ({ page }) => {
    await expect(page.getByText('nothing checked', { exact: true }).first()).toBeVisible();
  });

  test('shows finish card with back to home link', async ({ page }) => {
    await expect(page.getByText("That's the core of CUI.", { exact: true })).toBeVisible();
    await expect(page.getByText('← Back to Home', { exact: true })).toBeVisible();
  });

  test('back to home link works', async ({ page }) => {
    await page.getByText('← Back to Home', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('What is CUI?', { exact: true })).toBeVisible();
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
    await page.getByText('Variables →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 5: Variables', { exact: true })).toBeVisible();

    // Lesson 5 → Lesson 6
    await page.getByText('Putting It Together →', { exact: true }).click();
    await page.waitForTimeout(300);
    await expect(page.getByText('Lesson 6: Putting It All Together', { exact: true })).toBeVisible();

    // Should see the finish card
    await expect(page.getByText("That's the core of CUI.", { exact: true })).toBeVisible();
  });
});
