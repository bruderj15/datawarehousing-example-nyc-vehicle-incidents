# NYC Vehicle Incidents Data Warehouse Analysis (2016-2022)

This document provides a comprehensive analysis of the New York City vehicle incidents data warehouse. The analysis is specifically tailored to person-oriented granularity, utilizing the custom Key Performance Indicator (KPI) `Severity`, defined as `Injured + (10 * Killed)`. 

## 1. Setting Up the Visual Evaluation (SSAS & PowerPivot Guide)

Before diving into the SQL queries, visual analysis is crucial for validating our findings. Microsoft SQL Server Analysis Services (SSAS) and Excel PowerPivot provide excellent tools for slicing multidimensional data.

### 1.1 Defining the Custom KPI in SSAS
1. Open your SSAS Multidimensional Project in Visual Studio.
2. Navigate to the **Cube Designer** and click on the **Calculations** tab.
3. Click the **New Calculated Member** icon.
4. Set the **Name** to `[Measures].[Severity]`.
5. Set the **Expression** to:
   ```mdx
   [Measures].[PersonsInjured] + (10 * [Measures].[PersonsKilled])
   ```
   *(Note: Adjust the exact measure names based on how they are labeled in your cube).*
6. Under **Format String**, select `Standard` or `#,#`.
7. Deploy the cube to your SSAS instance.

### 1.2 Creating Visuals via Excel (Connecting to SSAS)
1. Open **Excel**, navigate to the **Data** tab -> **Get Data** -> **From Database** -> **From SQL Server Analysis Services**.
2. Enter your SSAS server name and select the deployed NYC Incidents Cube.
3. Once the connection is established, create a **PivotChart**.
4. In the PivotChart Fields pane, you will now see your Dimensions (`DimTime`, `DimPersonSex`, etc.) and Measures (including your new `Severity` calculation). 
5. For each question below, specific instructions are provided on how to drag and drop these fields to replicate the analysis visually.

---

## 2. In-Depth Analysis

### Q1: Among female crash participants, does injury severity differ across moon phases, and does this pattern persist after controlling for weather?

**Reasoning:**
To answer this, we must group our data by `hier_moon_phase` and `weather` while filtering exclusively for females. Because the total number of female participants might vary significantly depending on the weather (e.g., fewer people drive in storms), we must look at both the absolute `Severity` and the `Average Severity per Person` to avoid misleading conclusions driven simply by exposure volume.

**SQL Query:**
```sql
SELECT 
    t.hier_moon_phase,
    t.weather,
    COUNT(f.fact_id) AS total_female_participants,
    SUM(f.persons_injured + 10 * f.persons_killed) AS absolute_severity,
    CAST(SUM(f.persons_injured + 10 * f.persons_killed) AS FLOAT) / NULLIF(COUNT(f.fact_id), 0) AS avg_severity_per_person
FROM project_julian_bruder_kenana_saeed.Fact f
JOIN project_julian_bruder_kenana_saeed.DimTime t ON f.time_id = t.time_id
JOIN project_julian_bruder_kenana_saeed.DimPersonSex s ON f.person_sex_id = s.person_sex_id
WHERE s.person_sex = 'FEMALE'
GROUP BY 
    t.hier_moon_phase,
    t.weather
ORDER BY 
    t.weather, 
    avg_severity_per_person DESC;
```

**SSAS / PowerPivot Visual Guide:**
- **Filters:** `Person Sex` = 'FEMALE'
- **Rows/Axis:** `Moon Phase`
- **Columns/Legend:** `Weather`
- **Values:** `[Measures].[Severity]` (and create a calculated measure for `[Measures].[Severity] / [Measures].[Fact Count]` for the average).
- **Chart Type:** Clustered Bar Chart. This allows you to easily compare the severity across moon phases within each specific weather cluster.

---

### Q2: Does injury severity among crash participants vary systematically by weather conditions, independent of sex and age group?

**Reasoning:**
To prove independence from sex and age group, we stratify the severity measure across all three dimensions simultaneously (`weather`, `person_sex`, `person_age_hier_def_group`). If weather systematically impacts severity, we should see a consistent trend (e.g., 'RAINY_HEAVY' showing higher average severity) across *all* permutations of sex and age groups.

**SQL Query:**
```sql
SELECT 
    t.weather,
    s.person_sex,
    a.person_age_hier_def_group,
    COUNT(f.fact_id) AS total_participants,
    SUM(f.persons_injured + 10 * f.persons_killed) AS absolute_severity,
    CAST(SUM(f.persons_injured + 10 * f.persons_killed) AS FLOAT) / NULLIF(COUNT(f.fact_id), 0) AS avg_severity_per_person
FROM project_julian_bruder_kenana_saeed.Fact f
JOIN project_julian_bruder_kenana_saeed.DimTime t ON f.time_id = t.time_id
JOIN project_julian_bruder_kenana_saeed.DimPersonSex s ON f.person_sex_id = s.person_sex_id
JOIN project_julian_bruder_kenana_saeed.DimPersonAge a ON f.person_age_id = a.person_age_id
GROUP BY 
    t.weather,
    s.person_sex,
    a.person_age_hier_def_group
ORDER BY 
    t.weather, 
    s.person_sex, 
    a.person_age_hier_def_group;
```

**SSAS / PowerPivot Visual Guide:**
- **Filters:** None (include all data).
- **Rows/Axis:** `Weather`
- **Columns/Legend:** Hierarchical grouping: `Person Sex` -> `Age Group`.
- **Values:** `[Measures].[Average Severity per Person]` (Calculated Measure).
- **Chart Type:** Line Chart with Markers. If weather systematically drives severity, the lines for different demographics will move largely in parallel as they cross the weather categories on the X-axis.

---

### Q3: Is the distribution of crash severity among female participants stable across years, regardless of moon phase?

**Reasoning:**
We need to track temporal stability. By examining the average severity per year split by moon phase for female participants, we can determine if the variance between moon phases is noise or a consistent historical trend between 2016-2022.

**SQL Query:**
```sql
SELECT 
    t.hier_def_year,
    t.hier_moon_phase,
    COUNT(f.fact_id) AS incident_participation_count,
    SUM(f.persons_injured + 10 * f.persons_killed) AS absolute_severity,
    CAST(SUM(f.persons_injured + 10 * f.persons_killed) AS FLOAT) / NULLIF(COUNT(f.fact_id), 0) AS avg_severity_per_person
FROM project_julian_bruder_kenana_saeed.Fact f
JOIN project_julian_bruder_kenana_saeed.DimTime t ON f.time_id = t.time_id
JOIN project_julian_bruder_kenana_saeed.DimPersonSex s ON f.person_sex_id = s.person_sex_id
WHERE s.person_sex = 'FEMALE'
GROUP BY 
    t.hier_def_year,
    t.hier_moon_phase
ORDER BY 
    t.hier_def_year ASC, 
    t.hier_moon_phase;
```

**SSAS / PowerPivot Visual Guide:**
- **Filters:** `Person Sex` = 'FEMALE'
- **Rows/Axis:** `Year`
- **Columns/Legend:** `Moon Phase`
- **Values:** `[Measures].[Severity]` (or Average Severity).
- **Chart Type:** 100% Stacked Column Chart (if looking at absolute distribution of severity) or a Multi-line Chart (if looking at average severity stability). A flat horizontal trend across years for each moon phase indicates stability.

---

### Q4: Among male crash participants, does the distribution of contributing factor categories vary by moon phase?

**Reasoning:**
We want to see if the *reasons* for crashes among men change during different lunar phases. To do this, we calculate the percentage of total incidents each `contributing_factor_hier_def_category` accounts for within each distinct `moon_phase`. Using a window function (`OVER(PARTITION BY...)`) gives us the relative distribution.

**SQL Query:**
```sql
WITH PhaseTotals AS (
    SELECT 
        t.hier_moon_phase,
        COUNT(f.fact_id) AS phase_total_incidents
    FROM project_julian_bruder_kenana_saeed.Fact f
    JOIN project_julian_bruder_kenana_saeed.DimTime t ON f.time_id = t.time_id
    JOIN project_julian_bruder_kenana_saeed.DimPersonSex s ON f.person_sex_id = s.person_sex_id
    WHERE s.person_sex = 'MALE'
    GROUP BY t.hier_moon_phase
)
SELECT 
    t.hier_moon_phase,
    c.contributing_factor_hier_def_category,
    COUNT(f.fact_id) AS factor_incident_count,
    pt.phase_total_incidents,
    CAST(COUNT(f.fact_id) AS FLOAT) / NULLIF(pt.phase_total_incidents, 0) * 100 AS percentage_of_phase
FROM project_julian_bruder_kenana_saeed.Fact f
JOIN project_julian_bruder_kenana_saeed.DimTime t ON f.time_id = t.time_id
JOIN project_julian_bruder_kenana_saeed.DimPersonSex s ON f.person_sex_id = s.person_sex_id
JOIN project_julian_bruder_kenana_saeed.DimContributingFactor c ON f.contributing_factor_id = c.contributing_factor_id
JOIN PhaseTotals pt ON t.hier_moon_phase = pt.hier_moon_phase
WHERE s.person_sex = 'MALE'
GROUP BY 
    t.hier_moon_phase,
    c.contributing_factor_hier_def_category,
    pt.phase_total_incidents
ORDER BY 
    t.hier_moon_phase, 
    percentage_of_phase DESC;
```

**SSAS / PowerPivot Visual Guide:**
- **Filters:** `Person Sex` = 'MALE'
- **Rows/Axis:** `Moon Phase`
- **Columns/Legend:** `Contributing Factor Category`
- **Values:** `[Measures].[Fact Count]` 
- **Chart Type:** 100% Stacked Column Chart. Display values as "% of Grand Total" mapped to Row Total. This provides an immediate visual representation of whether the proportion of factors (e.g., 'SUBSTANCE_RELATED' vs 'HUMAN_BEHAVIOR') expands or shrinks during a full moon.

---

### Q5: Among female crash participants, injury and fatality severity distributions differ jointly across moon phase and weather conditions, after stratifying by age group.

**Reasoning:**
This is the most granular analysis, answering the primary analytics question modeled in the database schema. We are checking if the menstrual-cycle proxy (`FERTILE` vs `INFERTILE` female age groups) combined with `moon_phase` interacts with `weather` to produce distinct severity patterns. We break apart the KPI into its raw components (`injured` vs `killed`) alongside the combined `Severity` score to spot edge cases (e.g., phases with low injuries but high fatalities).

**SQL Query:**
```sql
SELECT 
    a.person_age_hier_def_group,
    t.hier_moon_phase,
    t.weather,
    COUNT(f.fact_id) AS total_participants,
    SUM(f.persons_injured) AS total_injured,
    SUM(f.persons_killed) AS total_killed,
    SUM(f.persons_injured + 10 * f.persons_killed) AS combined_severity,
    CAST(SUM(f.persons_injured + 10 * f.persons_killed) AS FLOAT) / NULLIF(COUNT(f.fact_id), 0) AS avg_severity_per_person
FROM project_julian_bruder_kenana_saeed.Fact f
JOIN project_julian_bruder_kenana_saeed.DimTime t ON f.time_id = t.time_id
JOIN project_julian_bruder_kenana_saeed.DimPersonSex s ON f.person_sex_id = s.person_sex_id
JOIN project_julian_bruder_kenana_saeed.DimPersonAge a ON f.person_age_id = a.person_age_id
WHERE s.person_sex = 'FEMALE'
  AND a.person_age_hier_def_group IN ('FERTILE', 'INFERTILE') -- Filter out 'UNKNOWN'
GROUP BY 
    a.person_age_hier_def_group,
    t.hier_moon_phase,
    t.weather
ORDER BY 
    a.person_age_hier_def_group, 
    t.hier_moon_phase, 
    t.weather;
```

**SSAS / PowerPivot Visual Guide:**
- **Filters:** `Person Sex` = 'FEMALE', `Age Group` = 'FERTILE', 'INFERTILE'
- **Rows/Axis:** `Age Group`, expanded down into `Moon Phase`.
- **Columns/Legend:** `Weather`
- **Values:** Two values side-by-side: `[Measures].[PersonsInjured]` and `[Measures].[PersonsKilled]`.
- **Chart Type:** Radar Chart (Spider Chart) or small multiples of Clustered Column Charts. A Radar chart is highly effective here: configure one radar web for 'FERTILE' and one for 'INFERTILE'. Plot the moon phases around the axes, and use different lines for weather conditions. Any massive divergence in shape between the two charts instantly proves the stratification hypothesis.
