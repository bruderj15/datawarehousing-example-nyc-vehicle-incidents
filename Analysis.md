# NYC Vehicle Incidents Data Warehouse Analysis (2016-2022)

This document provides a comprehensive analysis of the New York City vehicle incidents data
warehouse. The analysis is specifically tailored to person-oriented granularity, utilizing the
custom Key Performance Indicator (KPI) `Severity`, defined as `Injured + (10 * Killed)`.

## 1. Setting Up the Visual Evaluation (SSAS & PowerPivot Guide)

Before diving into the SQL queries, visual analysis is crucial for validating our findings.
Microsoft SQL Server Analysis Services (SSAS) and Excel PowerPivot provide excellent tools for
slicing multidimensional data.

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

1. Open **Excel**, navigate to the **Data** tab -> **Get Data** -> **From Database** -> **From SQL
   Server Analysis Services**.
2. Enter your SSAS server name and select the deployed NYC Incidents Cube.
3. Once the connection is established, create a **PivotChart**.
4. In the PivotChart Fields pane, you will now see your Dimensions (`DimTime`, `DimPersonSex`, etc.)
   and Measures (including your new `Severity` calculation).
5. For each question below, specific instructions are provided on how to drag and drop these fields
   to replicate the analysis visually.

---

## 2. In-Depth Analysis

### Q1: Among female crash participants, does injury severity differ across moon phases, and does this pattern persist after controlling for weather?

**Reasoning:**
To answer this, we must group our data by `hier_moon_phase` and `weather` while filtering
exclusively for females. Because the total number of female participants might vary significantly
depending on the weather (e.g., fewer people drive in storms), we must look at both the absolute
`Severity` and the `Average Severity per Person` to avoid misleading conclusions driven simply by
exposure volume.

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

**SQL Result:**

| hier_moon_phase | weather       | total_female_participants | absolute_severity | avg_severity_per_person |
|-----------------|---------------|--------------------------:|------------------:|------------------------:|
| LAST_QUARTER    | CLEAR         |                    107358 |             56044 |       0.522029098902737 |
| NEW             | CLEAR         |                    100292 |             51110 |       0.509611933155187 |
| FULL            | CLEAR         |                    107182 |             53630 |       0.500363867067231 |
| FIRST_QUARTER   | CLEAR         |                    115071 |             55267 |       0.480286084243641 |
| NEW             | CLOUDY        |                     91594 |             44805 |       0.489169596261764 |
| FIRST_QUARTER   | CLOUDY        |                     76804 |             37463 |       0.487774074266965 |
| FULL            | CLOUDY        |                     78147 |             37952 |       0.485648841286294 |
| LAST_QUARTER    | CLOUDY        |                     85244 |             39411 |       0.462331659706255 |
| LAST_QUARTER    | MISCALLANEOUS |                     89222 |             46817 |       0.524724843648428 |
| FIRST_QUARTER   | MISCALLANEOUS |                     88201 |             45587 |       0.516853550413261 |
| NEW             | MISCALLANEOUS |                     78048 |             40110 |       0.513914514145141 |
| FULL            | MISCALLANEOUS |                     93383 |             46019 |        0.49279847509718 |
| FULL            | RAINY_HEAVY   |                      3716 |              1834 |       0.493541442411195 |
| LAST_QUARTER    | RAINY_HEAVY   |                      3024 |              1482 |       0.490079365079365 |
| FIRST_QUARTER   | RAINY_HEAVY   |                      3170 |              1476 |       0.465615141955836 |
| NEW             | RAINY_HEAVY   |                      3559 |              1647 |        0.46277044113515 |
| FIRST_QUARTER   | RAINY_LIGHT   |                     45675 |             22868 |       0.500667761357417 |
| FULL            | RAINY_LIGHT   |                     45323 |             22416 |       0.494583324140061 |
| LAST_QUARTER    | RAINY_LIGHT   |                     40342 |             19453 |       0.482202171434237 |
| NEW             | RAINY_LIGHT   |                     47915 |             22525 |       0.470103307941146 |
| FIRST_QUARTER   | STORMY        |                        11 |                 9 |       0.818181818181818 |
| FULL            | STORMY        |                        62 |                33 |       0.532258064516129 |
| UNKNOWN         | UNKNOWN       |                       726 |               703 |        0.96831955922865 |
| FIRST_QUARTER   | UNKNOWN       |                      4224 |              3303 |       0.781960227272727 |
| FULL            | UNKNOWN       |                      4887 |              3808 |       0.779210149375895 |
| NEW             | UNKNOWN       |                      4911 |              3726 |       0.758704948075748 |
| LAST_QUARTER    | UNKNOWN       |                      5483 |              4157 |       0.758161590370235 |

**SSAS / PowerPivot Visual Guide:**

- **Filters:** `Person Sex` = 'FEMALE'
- **Rows/Axis:** `Moon Phase`
- **Columns/Legend:** `Weather`
- **Values:** `[Measures].[Severity]` (and create a calculated measure for
  `[Measures].[Severity] / [Measures].[Fact Count]` for the average).
- **Chart Type:** Clustered Bar Chart. This allows you to easily compare the severity across moon
  phases within each specific weather cluster.

---

### Q2: Does injury severity among crash participants vary systematically by weather conditions, independent of sex and age group?

**Reasoning:**
To prove independence from sex and age group, we stratify the severity measure across all three
dimensions simultaneously (`weather`, `person_sex`, `person_age_hier_def_group`). If weather
systematically impacts severity, we should see a consistent trend (e.g., 'RAINY_HEAVY' showing
higher average severity) across *all* permutations of sex and age groups.

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

**SQL Result:**

| weather       | person_sex | person_age_hier_def_group | total_participants | absolute_severity | avg_severity_per_person |
|---------------|------------|--------------------------:|-------------------:|------------------:|------------------------:|
| CLEAR         | 	FEMALE    |                  	FERTILE |            	259375 |           	132350 |      	0,510265060240964 |
| CLEAR         | 	FEMALE    |                	INFERTILE |            	137013 |            	70126 |      	0,511820046272981 |
| CLEAR         | 	FEMALE    |                  	UNKNOWN |             	33515 |            	13575 |      	0,405042518275399 |
| CLEAR         | 	MALE      |                  	FERTILE |            	515753 |           	226325 |      	0,438824398500833 |
| CLEAR         | 	MALE      |                	INFERTILE |            	278597 |           	114711 |      	0,411745280817812 |
| CLEAR         | 	MALE      |                  	UNKNOWN |             	55428 |            	20877 |      	0,376650790214332 |
| CLEAR         | 	UNKNOWN   |                  	FERTILE |              	5281 |             	5134 |      	0,972164362810074 |
| CLEAR         | 	UNKNOWN   |                	INFERTILE |              	3706 |             	3016 |      	0,813815434430653 |
| CLEAR         | 	UNKNOWN   |                  	UNKNOWN |            	246706 |            	91143 |      	0,369439737987726 |
| CLOUDY        | 	FEMALE    |                  	FERTILE |            	200050 |            	96586 |      	0,482809297675581 |
| CLOUDY        | 	FEMALE    |                	INFERTILE |            	105622 |            	52746 |      	0,499384597905739 |
| CLOUDY        | 	FEMALE    |                  	UNKNOWN |             	26117 |            	10299 |      	0,394340850786844 |
| CLOUDY        | 	MALE      |                  	FERTILE |            	397266 |           	164182 |      	0,413279767208873 |
| CLOUDY        | 	MALE      |                	INFERTILE |            	214356 |            	86128 |       	0,40179887663513 |
| CLOUDY        | 	MALE      |                  	UNKNOWN |             	42680 |            	15843 |      	0,371204311152765 |
| CLOUDY        | 	UNKNOWN   |                  	FERTILE |              	3635 |             	3169 |      	0,871801925722146 |
| CLOUDY        | 	UNKNOWN   |                	INFERTILE |              	2560 |             	1727 |            	0,674609375 |
| CLOUDY        | 	UNKNOWN   |                  	UNKNOWN |            	189053 |            	66039 |      	0,349314742426727 |
| MISCALLANEOUS | 	FEMALE    |                  	FERTILE |            	210380 |           	109234 |       	0,51922235953988 |
| MISCALLANEOUS | 	FEMALE    |                	INFERTILE |            	110216 |            	57897 |      	0,525304855919286 |
| MISCALLANEOUS | 	FEMALE    |                  	UNKNOWN |             	28258 |            	11402 |      	0,403496355014509 |
| MISCALLANEOUS | 	MALE      |                  	FERTILE |            	422209 |           	189700 |      	0,449303543979404 |
| MISCALLANEOUS | 	MALE      |                	INFERTILE |            	222112 |            	94650 |      	0,426136363636364 |
| MISCALLANEOUS | 	MALE      |                  	UNKNOWN |             	45238 |            	17106 |      	0,378133427649321 |
| MISCALLANEOUS | 	UNKNOWN   |                  	FERTILE |              	3904 |             	3642 |      	0,932889344262295 |
| MISCALLANEOUS | 	UNKNOWN   |                	INFERTILE |              	2809 |             	2115 |      	0,752936988252047 |
| MISCALLANEOUS | 	UNKNOWN   |                  	UNKNOWN |            	197735 |            	75231 |      	0,380463751991302 |
| RAINY_HEAVY   | 	FEMALE    |                  	FERTILE |              	8424 |             	4173 |       	0,49537037037037 |
| RAINY_HEAVY   | 	FEMALE    |                	INFERTILE |              	3995 |             	1866 |      	0,467083854818523 |
| RAINY_HEAVY   | 	FEMALE    |                  	UNKNOWN |              	1050 |              	400 |      	0,380952380952381 |
| RAINY_HEAVY   | 	MALE      |                  	FERTILE |             	16797 |             	6910 |      	0,411382985056855 |
| RAINY_HEAVY   | 	MALE      |                	INFERTILE |              	8327 |             	3330 |      	0,399903926984508 |
| RAINY_HEAVY   | 	MALE      |                  	UNKNOWN |              	1641 |              	545 |      	0,332114564290067 |
| RAINY_HEAVY   | 	UNKNOWN   |                  	FERTILE |               	100 |               	80 |                    	0,8 |
| RAINY_HEAVY   | 	UNKNOWN   |                	INFERTILE |                	95 |               	51 |      	0,536842105263158 |
| RAINY_HEAVY   | 	UNKNOWN   |                  	UNKNOWN |              	7738 |             	2704 |      	0,349444300852934 |
| RAINY_LIGHT   | 	FEMALE    |                  	FERTILE |            	109754 |            	53909 |      	0,491180275889717 |
| RAINY_LIGHT   | 	FEMALE    |                	INFERTILE |             	55003 |            	27681 |      	0,503263458356817 |
| RAINY_LIGHT   | 	FEMALE    |                  	UNKNOWN |             	14498 |             	5672 |      	0,391226376051869 |
| RAINY_LIGHT   | 	MALE      |                  	FERTILE |            	222849 |            	92515 |      	0,415146579073723 |
| RAINY_LIGHT   | 	MALE      |                	INFERTILE |            	114035 |            	46151 |      	0,404709080545447 |
| RAINY_LIGHT   | 	MALE      |                  	UNKNOWN |             	23149 |             	8378 |      	0,361916281480841 |
| RAINY_LIGHT   | 	UNKNOWN   |                  	FERTILE |              	1923 |             	1568 |      	0,815392615704628 |
| RAINY_LIGHT   | 	UNKNOWN   |                	INFERTILE |              	1287 |              	936 |      	0,727272727272727 |
| RAINY_LIGHT   | 	UNKNOWN   |                  	UNKNOWN |            	102891 |            	36542 |      	0,355152540066673 |
| STORMY        | 	FEMALE    |                  	FERTILE |                	52 |               	33 |      	0,634615384615385 |
| STORMY        | 	FEMALE    |                	INFERTILE |                	15 |                	9 |                    	0,6 |
| STORMY        | 	FEMALE    |                  	UNKNOWN |                 	6 |                	0 |                      	0 |
| STORMY        | 	MALE      |                  	FERTILE |                	97 |               	50 |      	0,515463917525773 |
| STORMY        | 	MALE      |                	INFERTILE |                	50 |               	23 |                   	0,46 |
| STORMY        | 	MALE      |                  	UNKNOWN |                 	5 |                	2 |                    	0,4 |
| STORMY        | 	UNKNOWN   |                  	FERTILE |                 	1 |                	0 |                      	0 |
| STORMY        | 	UNKNOWN   |                  	UNKNOWN |                	40 |               	16 |                    	0,4 |
| UNKNOWN       | 	FEMALE    |                  	FERTILE |             	12417 |             	9669 |      	0,778690504952887 |
| UNKNOWN       | 	FEMALE    |                	INFERTILE |              	6502 |             	5236 |      	0,805290679790834 |
| UNKNOWN       | 	FEMALE    |                  	UNKNOWN |              	1312 |              	792 |      	0,603658536585366 |
| UNKNOWN       | 	MALE      |                  	FERTILE |             	25609 |            	18133 |      	0,708071381155063 |
| UNKNOWN       | 	MALE      |                	INFERTILE |             	12991 |             	9074 |      	0,698483565545378 |
| UNKNOWN       | 	MALE      |                  	UNKNOWN |              	2269 |             	1394 |      	0,614367562802997 |
| UNKNOWN       | 	UNKNOWN   |                  	FERTILE |               	165 |               	80 |      	0,484848484848485 |
| UNKNOWN       | 	UNKNOWN   |                	INFERTILE |               	161 |               	58 |      	0,360248447204969 |
| UNKNOWN       | 	UNKNOWN   |                  	UNKNOWN |              	9707 |             	5791 |      	0,596579787782013 |

**SSAS / PowerPivot Visual Guide:**

- **Filters:** None (include all data).
- **Rows/Axis:** `Weather`
- **Columns/Legend:** Hierarchical grouping: `Person Sex` -> `Age Group`.
- **Values:** `[Measures].[Average Severity per Person]` (Calculated Measure).
- **Chart Type:** Line Chart with Markers. If weather systematically drives severity, the lines for
  different demographics will move largely in parallel as they cross the weather categories on the
  X-axis.

---

### Q3: Is the distribution of crash severity among female participants stable across years, regardless of moon phase?

**Reasoning:**
We need to track temporal stability. By examining the average severity per year split by moon phase
for female participants, we can determine if the variance between moon phases is noise or a
consistent historical trend between 2016-2022.

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

**SQL Result:**

| hier_def_year | hier_moon_phase | incident_participation_count | absolute_severity | avg_severity_per_person |
|---------------|-----------------|-----------------------------:|------------------:|------------------------:|
| 2016          | 	FIRST_QUARTER  |                       	57011 |            	24873 |      	0,436284225851152 |
| 2016          | 	FULL           |                       	55154 |            	23014 |      	0,417268013199405 |
| 2016          | 	LAST_QUARTER   |                       	55342 |            	22662 |      	0,409490079867009 |
| 2016          | 	NEW            |                       	55229 |            	23360 |       	0,42296619529595 |
| 2017          | 	FIRST_QUARTER  |                       	71561 |            	29509 |      	0,412361481812719 |
| 2017          | 	FULL           |                       	66492 |            	27758 |        	0,4174637550382 |
| 2017          | 	LAST_QUARTER   |                       	66693 |            	27725 |      	0,415710794236277 |
| 2017          | 	NEW            |                       	67617 |            	28656 |      	0,423798748835352 |
| 2018          | 	FIRST_QUARTER  |                       	64726 |            	26998 |      	0,417112134227358 |
| 2018          | 	FULL           |                       	69867 |            	28983 |      	0,414831036111469 |
| 2018          | 	LAST_QUARTER   |                       	65998 |            	28120 |      	0,426073517379315 |
| 2018          | 	NEW            |                       	65054 |            	26693 |      	0,410320656685215 |
| 2019          | 	FIRST_QUARTER  |                       	57651 |            	26086 |      	0,452481309951258 |
| 2019          | 	FULL           |                       	58602 |            	25705 |      	0,438636906590219 |
| 2019          | 	LAST_QUARTER   |                       	62417 |            	29032 |      	0,465129692231283 |
| 2019          | 	NEW            |                       	59112 |            	28626 |      	0,484267153877385 |
| 2020          | 	FIRST_QUARTER  |                       	30028 |            	18259 |      	0,608065805248435 |
| 2020          | 	FULL           |                       	29980 |            	19816 |      	0,660973982655103 |
| 2020          | 	LAST_QUARTER   |                       	26634 |            	17195 |      	0,645603364121048 |
| 2020          | 	NEW            |                       	26121 |            	16703 |      	0,639447188086214 |
| 2021          | 	FIRST_QUARTER  |                       	26578 |            	20036 |      	0,753856573105576 |
| 2021          | 	FULL           |                       	28029 |            	21112 |       	0,75321987941061 |
| 2021          | 	LAST_QUARTER   |                       	27745 |            	21377 |      	0,770481167777978 |
| 2021          | 	NEW            |                       	26534 |            	19256 |      	0,725710409286199 |
| 2022          | 	FIRST_QUARTER  |                       	25601 |            	20212 |      	0,789500410140229 |
| 2022          | 	FULL           |                       	24576 |            	19304 |      	0,785481770833333 |
| 2022          | 	LAST_QUARTER   |                       	25844 |            	21253 |      	0,822357220244544 |
| 2022          | 	NEW            |                       	26652 |            	20629 |      	0,774013207263995 |
| 2022          | 	UNKNOWN        |                          726 |               703 |       	0,96831955922865 |

**SSAS / PowerPivot Visual Guide:**

- **Filters:** `Person Sex` = 'FEMALE'
- **Rows/Axis:** `Year`
- **Columns/Legend:** `Moon Phase`
- **Values:** `[Measures].[Severity]` (or Average Severity).
- **Chart Type:** 100% Stacked Column Chart (if looking at absolute distribution of severity) or a
  Multi-line Chart (if looking at average severity stability). A flat horizontal trend across years
  for each moon phase indicates stability.

---

### Q4: Among male crash participants, does the distribution of contributing factor categories vary by moon phase?

**Reasoning:**
We want to see if the *reasons* for crashes among men change during different lunar phases. To do
this, we calculate the percentage of total incidents each `contributing_factor_hier_def_category`
accounts for within each distinct `moon_phase`. Using a window function (`OVER(PARTITION BY...)`)
gives us the relative distribution.

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

**SQL Result:**

| hier_moon_phase | contributing_factor_hier_def_category | factor_incident_count | phase_total_incidents | percentage_of_phase |
|-----------------|---------------------------------------|----------------------:|----------------------:|--------------------:|
| FIRST_QUARTER   | 	HUMAN_BEHAVIOR                       |               	273223 |               	661859 |   	41,2811490060572 |
| FIRST_QUARTER   | 	DISTRACTION                          |               	172101 |               	661859 |   	26,0026682420274 |
| FIRST_QUARTER   | 	UNKNOWN                              |               	128753 |               	661859 |   	19,4532370187608 |
| FIRST_QUARTER   | 	VEHICLE_DEFECT                       |                	29335 |               	661859 |   	4,43221290335253 |
| FIRST_QUARTER   | 	EXTERNAL                             |                	16527 |               	661859 |   	2,49705753038034 |
| FIRST_QUARTER   | 	HUMAN_CONDITION                      |                	15488 |               	661859 |   	2,34007545413751 |
| FIRST_QUARTER   | 	ROAD_INFRASTRUCTURE                  |                	13236 |               	661859 |   	1,99982171429262 |
| FIRST_QUARTER   | 	SUBSTANCE_RELATED                    |                 	9954 |               	661859 |   	1,50394570444762 |
| FIRST_QUARTER   | 	ENVIRONMENTAL                        |                 	3242 |               	661859 |  	0,489832426544022 |
| FULL            | 	HUMAN_BEHAVIOR                       |               	270478 |               	660799 |    	40,931962669435 |
| FULL            | 	DISTRACTION                          |               	172047 |               	660799 |   	26,0362076819123 |
| FULL            | 	UNKNOWN                              |               	129437 |               	660799 |   	19,5879533716001 |
| FULL            | 	VEHICLE_DEFECT                       |                	29227 |               	660799 |   	4,42297884833361 |
| FULL            | 	EXTERNAL                             |                	16322 |               	660799 |   	2,47004005756667 |
| FULL            | 	HUMAN_CONDITION                      |                	15952 |               	660799 |   	2,41404723675429 |
| FULL            | 	ROAD_INFRASTRUCTURE                  |                	14169 |               	660799 |   	2,14422237321788 |
| FULL            | 	SUBSTANCE_RELATED                    |                 	9964 |               	660799 |   	1,50787153128258 |
| FULL            | 	ENVIRONMENTAL                        |                 	3203 |               	660799 |  	0,484716229897442 |
| LAST_QUARTER    | 	HUMAN_BEHAVIOR                       |               	268244 |               	651110 |   	41,1979542627206 |
| LAST_QUARTER    | 	DISTRACTION                          |               	169474 |               	651110 |   	26,0284744513216 |
| LAST_QUARTER    | 	UNKNOWN                              |               	126249 |               	651110 |   	19,3898112454117 |
| LAST_QUARTER    | 	VEHICLE_DEFECT                       |                	29246 |               	651110 |   	4,49171414968285 |
| LAST_QUARTER    | 	EXTERNAL                             |                	16049 |               	651110 |   	2,46486768748752 |
| LAST_QUARTER    | 	HUMAN_CONDITION                      |                	15821 |               	651110 |   	2,42985056288492 |
| LAST_QUARTER    | 	ROAD_INFRASTRUCTURE                  |                	13033 |               	651110 |   	2,00165870590223 |
| LAST_QUARTER    | 	SUBSTANCE_RELATED                    |                 	9966 |               	651110 |    	1,5306169464453 |
| LAST_QUARTER    | 	ENVIRONMENTAL                        |                 	3028 |               	651110 |  	0,465051988143324 |
| NEW             | 	HUMAN_BEHAVIOR                       |               	266753 |               	646175 |   	41,2818508917863 |
| NEW             | 	DISTRACTION                          |               	168940 |               	646175 |    	26,144620265408 |
| NEW             | 	UNKNOWN                              |               	124882 |               	646175 |   	19,3263434828026 |
| NEW             | 	VEHICLE_DEFECT                       |                	28744 |               	646175 |   	4,44833056060665 |
| NEW             | 	EXTERNAL                             |                	15513 |               	646175 |   	2,40074283282393 |
| NEW             | 	HUMAN_CONDITION                      |                	15299 |               	646175 |   	2,36762486942392 |
| NEW             | 	ROAD_INFRASTRUCTURE                  |                	12958 |               	646175 |   	2,00533911092196 |
| NEW             | 	SUBSTANCE_RELATED                    |                	10110 |               	646175 |   	1,56459163539289 |
| NEW             | 	ENVIRONMENTAL                        |                 	2976 |               	646175 |  	0,460556350833752 |
| UNKNOWN         | 	HUMAN_BEHAVIOR                       |                  	591 |                 	1515 |    	39,009900990099 |
| UNKNOWN         | 	DISTRACTION                          |                  	390 |                 	1515 |   	25,7425742574257 |
| UNKNOWN         | 	UNKNOWN                              |                  	271 |                 	1515 |   	17,8877887788779 |
| UNKNOWN         | 	VEHICLE_DEFECT                       |                   	80 |                 	1515 |   	5,28052805280528 |
| UNKNOWN         | 	SUBSTANCE_RELATED                    |                   	75 |                 	1515 |   	4,95049504950495 |
| UNKNOWN         | 	HUMAN_CONDITION                      |                   	44 |                 	1515 |    	2,9042904290429 |
| UNKNOWN         | 	ROAD_INFRASTRUCTURE                  |                   	32 |                 	1515 |   	2,11221122112211 |
| UNKNOWN         | 	EXTERNAL                             |                   	30 |                 	1515 |   	1,98019801980198 |
| UNKNOWN         | 	ENVIRONMENTAL                        |                    	2 |                 	1515 |  	0,132013201320132 |

**SSAS / PowerPivot Visual Guide:**

- **Filters:** `Person Sex` = 'MALE'
- **Rows/Axis:** `Moon Phase`
- **Columns/Legend:** `Contributing Factor Category`
- **Values:** `[Measures].[Fact Count]`
- **Chart Type:** 100% Stacked Column Chart. Display values as "% of Grand Total" mapped to Row
  Total. This provides an immediate visual representation of whether the proportion of factors (
  e.g., 'SUBSTANCE_RELATED' vs 'HUMAN_BEHAVIOR') expands or shrinks during a full moon.

---

### Q5: Among female crash participants, injury and fatality severity distributions differ jointly across moon phase and weather conditions, after stratifying by age group.

**Reasoning:**
This is the most granular analysis, answering the primary analytics question modeled in the database
schema. We are checking if the menstrual-cycle proxy (`FERTILE` vs `INFERTILE` female age groups)
combined with `moon_phase` interacts with `weather` to produce distinct severity patterns. We break
apart the KPI into its raw components (`injured` vs `killed`) alongside the combined `Severity`
score to spot edge cases (e.g., phases with low injuries but high fatalities).

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

**SQL Result:**

| person_age_hier_def_group | hier_moon_phase |       weather | total_participants | total_injured | total_killed | combined_severity | avg_severity_per_person |
|---------------------------|-----------------|--------------:|-------------------:|--------------:|--------------|------------------:|------------------------:|
| FERTILE                   | FIRST_QUARTER   |         CLEAR |              68957 |         32365 | 96           |             33325 |       0,483272184114738 |
| FERTILE                   | FIRST_QUARTER   |        CLOUDY |              46328 |         22304 | 52           |             22824 |        0,49266102572958 |
| FERTILE                   | FIRST_QUARTER   | MISCALLANEOUS |              52888 |         26859 | 96           |             27819 |       0,525998336106489 |
| FERTILE                   | FIRST_QUARTER   |   RAINY_HEAVY |               1985 |           863 | 7            |               933 |       0,470025188916877 |
| FERTILE                   | FIRST_QUARTER   |   RAINY_LIGHT |              28045 |         13910 | 27           |             14180 |       0,505615974326974 |
| FERTILE                   | FIRST_QUARTER   |        STORMY |                  8 |             9 | 0            |                 9 |                   1,125 |
| FERTILE                   | FIRST_QUARTER   |       UNKNOWN |               2556 |          2000 | 7            |              2070 |       0,809859154929577 |
| FERTILE                   | FULL            |         CLEAR |              64776 |         32428 | 101          |             33438 |          0,516209707299 |
| FERTILE                   | FULL            |        CLOUDY |              47015 |         22522 | 34           |             22862 |        0,48627033925343 |
| FERTILE                   | FULL            | MISCALLANEOUS |              56132 |         27289 | 62           |             27909 |       0,497203021449441 |
| FERTILE                   | FULL            |   RAINY_HEAVY |               2362 |          1131 | 10           |              1231 |        0,52116850127011 |
| FERTILE                   | FULL            |   RAINY_LIGHT |              27686 |         13699 | 38           |             14079 |       0,508524163837318 |
| FERTILE                   | FULL            |        STORMY |                 44 |            24 | 0            |                24 |       0,545454545454545 |
| FERTILE                   | FULL            |       UNKNOWN |               2993 |          2249 | 4            |              2289 |        0,76478449716004 |
| FERTILE                   | LAST_QUARTER    |         CLEAR |              64850 |         33214 | 88           |             34094 |       0,525736314572089 |
| FERTILE                   | LAST_QUARTER    |        CLOUDY |              51608 |         23729 | 30           |             24029 |        0,46560610758022 |
| FERTILE                   | LAST_QUARTER    | MISCALLANEOUS |              53791 |         27649 | 90           |             28549 |       0,530739343012772 |
| FERTILE                   | LAST_QUARTER    |   RAINY_HEAVY |               1845 |           847 | 7            |               917 |       0,497018970189702 |
| FERTILE                   | LAST_QUARTER    |   RAINY_LIGHT |              24838 |         11862 | 19           |             12052 |        0,48522425316048 |
| FERTILE                   | LAST_QUARTER    |       UNKNOWN |               3374 |          2496 | 1            |              2506 |       0,742738589211618 |
| FERTILE                   | NEW             |         CLEAR |              60792 |         30533 | 96           |             31493 |       0,518045137518094 |
| FERTILE                   | NEW             |        CLOUDY |              55099 |         26111 | 76           |             26871 |        0,48768580192018 |
| FERTILE                   | NEW             | MISCALLANEOUS |              47569 |         24387 | 57           |             24957 |       0,524648405474153 |
| FERTILE                   | NEW             |   RAINY_HEAVY |               2232 |          1042 | 5            |              1092 |       0,489247311827957 |
| FERTILE                   | NEW             |   RAINY_LIGHT |              29185 |         13228 | 37           |             13598 |       0,465924276169265 |
| FERTILE                   | NEW             |       UNKNOWN |               3042 |          2210 | 13           |              2340 |       0,769230769230769 |
| FERTILE                   | UNKNOWN         |       UNKNOWN |                452 |           404 | 6            |               464 |        1,02654867256637 |
| INFERTILE                 | FIRST_QUARTER   |         CLEAR |              36521 |         17250 | 81           |             18060 |       0,494510007940637 |
| INFERTILE                 | FIRST_QUARTER   |        CLOUDY |              24600 |         11883 | 48           |             12363 |       0,502560975609756 |
| INFERTILE                 | FIRST_QUARTER   | MISCALLANEOUS |              27941 |         13977 | 71           |             14687 |       0,525643319852546 |
| INFERTILE                 | FIRST_QUARTER   |   RAINY_HEAVY |                976 |           432 | 2            |               452 |       0,463114754098361 |
| INFERTILE                 | FIRST_QUARTER   |   RAINY_LIGHT |              14081 |          6936 | 28           |              7216 |       0,512463603437256 |
| INFERTILE                 | FIRST_QUARTER   |        STORMY |                  2 |             0 | 0            |                 0 |                       0 |
| INFERTILE                 | FIRST_QUARTER   |       UNKNOWN |               1381 |           996 | 4            |              1036 |       0,750181028240406 |
| INFERTILE                 | FULL            |         CLEAR |              34025 |         16217 | 77           |             16987 |       0,499250551065393 |
| INFERTILE                 | FULL            |        CLOUDY |              24584 |         12205 | 36           |             12565 |       0,511104783599089 |
| INFERTILE                 | FULL            | MISCALLANEOUS |              29594 |         14861 | 40           |             15261 |       0,515678853821721 |
| INFERTILE                 | FULL            |   RAINY_HEAVY |               1084 |           481 | 2            |               501 |       0,462177121771218 |
| INFERTILE                 | FULL            |   RAINY_LIGHT |              14024 |          6772 | 20           |              6972 |       0,497147746719909 |
| INFERTILE                 | FULL            |        STORMY |                 13 |             9 | 0            |                 9 |       0,692307692307692 |
| INFERTILE                 | FULL            |       UNKNOWN |               1584 |          1290 | 3            |              1320 |       0,833333333333333 |
| INFERTILE                 | LAST_QUARTER    |         CLEAR |              34580 |         17774 | 80           |             18574 |       0,537131289762869 |
| INFERTILE                 | LAST_QUARTER    |        CLOUDY |              26977 |         12271 | 57           |             12841 |       0,475998072432072 |
| INFERTILE                 | LAST_QUARTER    | MISCALLANEOUS |              28340 |         14825 | 51           |             15335 |       0,541107974594213 |
| INFERTILE                 | LAST_QUARTER    |   RAINY_HEAVY |                907 |           418 | 4            |               458 |       0,504961411245865 |
| INFERTILE                 | LAST_QUARTER    |   RAINY_LIGHT |              12058 |          5699 | 27           |              5969 |       0,495024050422956 |
| INFERTILE                 | LAST_QUARTER    |       UNKNOWN |               1765 |          1434 | 3            |              1464 |       0,829461756373938 |
| INFERTILE                 | NEW             |         CLEAR |              31887 |         15975 | 53           |             16505 |       0,517609056982469 |
| INFERTILE                 | NEW             |        CLOUDY |              29461 |         14327 | 65           |             14977 |       0,508366993652625 |
| INFERTILE                 | NEW             | MISCALLANEOUS |              24341 |         12084 | 53           |             12614 |       0,518220286758966 |
| INFERTILE                 | NEW             |   RAINY_HEAVY |               1028 |           455 | 0            |               455 |       0,442607003891051 |
| INFERTILE                 | NEW             |   RAINY_LIGHT |              14840 |          7324 | 20           |              7524 |       0,507008086253369 |
| INFERTILE                 | NEW             |       UNKNOWN |               1555 |          1159 | 5            |              1209 |       0,777491961414791 |
| INFERTILE                 | UNKNOWN         |       UNKNOWN |                217 |           167 | 4            |               207 |       0,953917050691244 |

**SSAS / PowerPivot Visual Guide:**

- **Filters:** `Person Sex` = 'FEMALE', `Age Group` = 'FERTILE', 'INFERTILE'
- **Rows/Axis:** `Age Group`, expanded down into `Moon Phase`.
- **Columns/Legend:** `Weather`
- **Values:** Two values side-by-side: `[Measures].[PersonsInjured]` and
  `[Measures].[PersonsKilled]`.
- **Chart Type:** Radar Chart (Spider Chart) or small multiples of Clustered Column Charts. A Radar
  chart is highly effective here: configure one radar web for 'FERTILE' and one for 'INFERTILE'.
  Plot the moon phases around the axes, and use different lines for weather conditions. Any massive
  divergence in shape between the two charts instantly proves the stratification hypothesis.
