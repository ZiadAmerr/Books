# Chapter 1 - Fundamentals of Testing

## Table of Contents

- [Chapter 1 - Fundamentals of Testing](#chapter-1---fundamentals-of-testing)
  - [Table of Contents](#table-of-contents)
  - [1.1 What is Testing?](#11-what-is-testing)
    - [1.1.1 Test Objectives](#111-test-objectives)
    - [1.1.2 Testing and Debugging](#112-testing-and-debugging)
  - [1.2 Why is Testing Necessary?](#12-why-is-testing-necessary)
    - [1.2.1 Testing's Contribution to Success](#121-testings-contribution-to-success)
      - [Product Quality](#product-quality)
      - [Process Quality](#process-quality)
      - [Project Goals](#project-goals)
      - [People Skills](#people-skills)
    - [1.2.2 Testing and Quality Assurance](#122-testing-and-quality-assurance)
      - [Quality Assurance](#quality-assurance)
      - [Quality Control](#quality-control)
      - [Summary of Quality Assurance and Quality Control](#summary-of-quality-assurance-and-quality-control)
    - [1.2.3 Errors, Defects, Failures, and Root Causes](#123-errors-defects-failures-and-root-causes)
  - [1.3 Testing Principles](#13-testing-principles)
    - [1. Testing shows the presence, not the absence, of defects](#1-testing-shows-the-presence-not-the-absence-of-defects)
    - [2. Exhaustive testing is impossible](#2-exhaustive-testing-is-impossible)
    - [3. Early testing saves time and money](#3-early-testing-saves-time-and-money)
    - [4. Defects cluster together](#4-defects-cluster-together)
    - [5. Tests wear out. (or the "Pesticide Paradox")](#5-tests-wear-out-or-the-pesticide-paradox)
    - [6. Testing is context dependent](#6-testing-is-context-dependent)
    - [7. Absence-of-errors fallacy](#7-absence-of-errors-fallacy)
  - [1.4 Test Activities, Testware, and Test Roles](#14-test-activities-testware-and-test-roles)
    - [1.4.1 Test Activities and Tasks](#141-test-activities-and-tasks)
      - [Test Planning](#test-planning)
      - [Test Monitoring and Test Control](#test-monitoring-and-test-control)
      - [Test Analysis](#test-analysis)
      - [Test Design](#test-design)
      - [Test Implementation](#test-implementation)

## 1.1 What is Testing?

**Testing** of software enables the assessment of software quality and contributes to reducing the risk of software failure in action. Therefore, good testing is essential for project success.

Software testing is a set of activities carried out to facilitate the detection of defects and evaluate properties of software artifacts. Each artifact is known as the **test object**.

Testing is mistaken to be just executing tests. Other activities are:

1. Test planning
2. Test monitoring and control
3. Test analysis
4. Test design
5. Test implementation
6. Test execution
7. Test completion

Testing is often seen as an activity focused solely on **verification** of requirements, user stories, or other forms of specification. Testing also includes **validation**, that is, verifying that the system meets user requirements and other stakeholder needs in its operational environment.

Testing can be performed without running the object under test, this is known as **static testing**.

Testing thus also includes reviews of work products such as:

- Requirements
- User stories
- Source code

Dynamic testing uses different types of test techniques (e.g., black-box, white-box, and experience-based) to derive test cases.

Testing is not just a technical activity. Testing must be properly planned, managed, estimated, monitored, and controlled.

Testing is largely an *intellectual, sapient* activity, requiring testers to have specialized knowledge, analytical skills, critical thinking, and systems thinking.

Testing is a *technical study* to obtain information about the quality of the test object:

- *Technical*: Because we use an engineering approach, using experiments, experience, formal techniques, mathematics, logic, tools (supporting programs), measurements, etc.
- *Study*: Because it is a continuous organized search for information

### 1.1.1 Test Objectives

Primary test objectives are:

- Evaluating work products such as requirements, user stories, designs, and code
- Triggering failures and finding defects
- Ensuring required coverage of a test object
- Reducing the level of risk of inadequate software quality
- Verifying whether specified requirements have been fulfilled
- Verifying that a test object complies with contractual, legal, and regulatory requirements
- Providing information to stakeholders to allow them to make informed decisions
- Building confidence in the quality of the test object
- Validating whether the test object is complete and works as expected by the stakeholders

### 1.1.2 Testing and Debugging

Testing and debugging are two different activities:

- **Testing** is supposed to reveal failures caused by defects.
- **Debugging** is a programming activity performed to identify the cause of a **defect** (fault), correcting the code and verifying that the defect has been correctly fixed.

When dynamic tests detect a failure, a typical debugging process will consist of:

- Failure reproduction
  - To make sure the failure actually occurs and so that it can be triggered in a controlled manner in the subsequent debugging process
- Diagnosis
  - Finding the case of the failure, such as locating the defect responsible for the occurrence of the failure
- Fixing the cause
  - Eliminating the cause of failure, such as fixing a defect in the code

The subsequent confirmation testing (re-testing) is performed by the tester to ensure that the fix actually fixed the failure. Most often, confirmation testing is performed by the same person who performed the original test that revealed the problem.

Regression testing can also be performed after the fix to verify that the fix in the code did not cause the software to malfunction elsewhere.

When static testing discovers a defect, the debugging process is simply to eliminate the defect. It is not necessary, as in the case of failure discovery in dynamic testing, to perform failure reproduction and diagnosis, because in static testing, the work product under test is not run. The related source code might not even have been created. This is because static testing does nt find failures but directly identifies defects.

## 1.2 Why is Testing Necessary?

Testing of components, systems, and related documentation supports the identification of software defects.

Testing also detects gaps and other deficiencies in software specifications. Hence, testing can help reduce the risk of failure during operation.

When detected defects are fixed, this contributes to improving the quality of the test object. In addition, software testing may also be required to meet contractual or legal requirements or to meet regulatory standards.

When there is an anomaly in an application, two questions must always be answered:

- What is the probability of its occurrence?
- What is the impact of this problem?

### 1.2.1 Testing's Contribution to Success

Testing helps achieve the agreed objectives within the agreed scope, time, quality, and budget. The contribution of testing to project success can be considered in terms of:

- Product quality
- Process quality
- Project goals
- People skills

#### Product Quality

Testing provides a cost-effective means of detecting defects. Rigorous testing of systems and documentation can reduce the risk of problems (failure) in the production environment and contribute to achieving high product **quality**.

#### Process Quality

Testing can indirectly contribute to the quality of the manufacturing process. It is known that the final quality of a product is very much influenced by the quality of the process by which the product is developed.

#### Project Goals

Testing can help increase the likelihood of achieving project goals. For example, using static testing early in the project reduces software maintenance costs and improves developer efficiency by reducing the time spent on defect fixes.

Testing also provides a means of directly evaluating the quality of a test object at various stages in the Software Development Lifecycle (SDLC). These measures are used as part of a larger project management activity, contributing to decisions to move to the next stage of the SDLC.

#### People Skills

A "side effect" of testing is to increase the skills of team members and other stakeholders. for example, performing code reviews increases understanding of the code and allows less experienced developers to improve their programming and design skills. Close cooperation between testers and system architects during the design phase of the work enables both parties to better understand the project.

### 1.2.2 Testing and Quality Assurance

Testing is often mistakenly equated with *quality assurance* (QA). These are two separate (though related) processes tht are included in the broader term "Quality Management" (QM). QM encompasses all activities aimed at guiding and overseeing an organization's quality activities. The two basic elements of QM are:

- Quality Assurance
- Quality Control

#### Quality Assurance

**Quality Assurance** focuses on establishing, implementing, monitoring, improving, and adhering to quality-related processes. It works on the basis that if a good process is followed correctly, then it will generate a good product. When the relevant processes are implemented correctly, it contributes to defect prevention and increases confidence that appropriate levels of work product quality will be achieved. QA, when applied to software development and maintenance, should also be applied to software testing, which is part of the each of these activities. In addition, the use of root cause analysis to detect causes of defects and the application of lessons learned from retrospective meetings to improve processes are also important for effect quality assurance.

#### Quality Control

Quality Control encompasses a range of activities, including testing activities that support the achievement of appropriate quality levels. Testing activities are an important part of the overall software development or maintenance process. The proper conduct of QC, especially testing activities, is important for quality assurance, and quality assurance supports proper testing. This is because the test results are used by both quality assurance and quality control. In quality control, they are used to fix defects, while in quality assurance, they provide feedback on how well the development and test processes are performing.

#### Summary of Quality Assurance and Quality Control

|Category|Quality Assurance|Quality Control|
|---|---|---|
|General description|Implementing processes, methodologies, and standards to ensure that the developed product meets the required quality standards|Performing activities to verify that the developed product meets the required quality standards|
|Target|Improving the manufacturing process|Product improvement through failure and defect detection|
|Type of process|Preventative (defect prevention)|Control (defect detection), reactive|
|Examples of activities|Implementation of processes, e.g., defect management, change management, software release; quality audits; process and product measurements; verification of correct implementation and execution of processes; training of team members; selection of tools|Static analysis of project documentation; code reviews; analysis, design, implementation of test cases; dynamic testing; writing and executing test scripts; defect reporting; using tools to support testing|

### 1.2.3 Errors, Defects, Failures, and Root Causes

ISTQB approach distinguishes between three stages leading to an abnormal results, related to three very important concepts, which are:

- **Error**: a human action that causes an incorrect result
- **Defect**: an imperfection or defect in a work product that involves failure to meet requirements
- **Failure**: an event in which a component or system fails to perform a required function within a specific context

As a result of human **error** in software code or other related work product, a defect can be introduced to the work product.
> This is important to pay attention to during the exam, as it is counter-intuitive and falls under the vocabulary that the exam is required to know.

Usually, in common speech, a defect is called an error as we often say "There is an error at this place in the code." However, from the point of view of ISTQB terminology, this is incorrect. There may be a *defect* in the program. **An *error* always refers to a *human mistake***. Running a piece of code where there is a ***defect* may or may not cause a failure**.

The consecutive occurrence of three factors (error, defect, failure) causes the observed malfunction of the product under testing.

Errors can occur for many reasons:

- Time pressure
- Human fallibility
- Lack of experience or insufficient skills of project team members
- Problems with information exchange among stakeholders
- Ambiguities regarding understanding of requirements and project documentation
- Complexity of the code, design, architecture, problem being solved, and/or technology being used
- Misunderstandings about interfaces within and between systems, especially when there are a large number of them
- Using new, unfamiliar technologies

Failures in turn can be caused *not necessarily* by human error, but also by environmental factors, such as:

- Radiation
- Electromagnetic field
- Contamination

These factors can cause failures in the embedded software or affect the software's performance by changing hardware operating conditions.

Not all unexpected **test results** mean failures. A *false-positive* result can be the result of errors related to test execution, defects in test data, test environment, other testware, etc. False-positive results are reported as defects that are not actually there. Similar problems can cause the opposite situation, a *false-negative* result is a situation in which tests fail to detect a defect that should detect.

Test cases should be designed to avoid **defect masking**, that is, situations in which the occurrence of one defect prevents the detection of another defect or the occurrence of two defects cancels their mutual effect.

In connection with the above, an important factor is the analysis of the **root cause** of the defect: the primary reason that resulted in the defect.

**Example** A defect in the code causes incorrect calculation of the discount on bulk purchases in the e-store, resulting in customer complaints. The defective code was written based on a user story, but the product owner misunderstood the discount calculation rules and wrote the story wrong.

In this example:

- Customer complaints are the *consequences*.
- Incorrect calculation of discounts is a *failure*.
- Incorrect discount calculation formula implemented in the code is a *defect*.
- Product owner's lack of knowledge is the *root cause*.
- The root cause is a result of the product owner's *error*.

## 1.3 Testing Principles

Foundation Level syllabus describes seven testing principles. Those are:

1. Testing shows the presence, not the absence, of defects.
2. Exhaustive testing is impossible.
3. Early testing saves time and money.
4. Defects cluster together.
5. Tests wear out.
6. Testing is context dependent.
7. Absence-of-defects fallacy.

### 1. Testing shows the presence, not the absence, of defects

While testing can show that defects exist, we can not prove that there are no defects in the program under testing. Thus, testing only reduces the likelihood that unidentified defects will remain in the software. The fact that defects are not detected is not a proof of the correctness of the system under test.

Testing is *negative* in nature, i.e., it shows that something does not work, not that everything is fine. This carries with it some important psychological implications.

### 2. Exhaustive testing is impossible

Exhaustive testing is the process of testing all possible inputs and preconditions for a program. This is not feasible in practice, especially for complex systems, because the number of possible inputs and preconditions is usually too large to test them all.

Instead, testing should focus on the most important and risky areas of the system, using risk analysis and prioritization techniques to determine which tests are most valuable.

### 3. Early testing saves time and money

Early testing is some times called *shift-left*. Test activities should start as early as possible for the software under test. This saves time and money, because defects that are fixed early in the process will not cause subsequent defects in derived work products such as design or code.

We should perform static testing, documentation reviews, and design reviews before the software is implemented. This allows us to detect defects early, when they are cheaper to fix.

### 4. Defects cluster together

Defects are not evenly distributed either in the software or over time. Most of the defects found in prerelease testing of software or that cause production failures are found in a small number of components. As a result, the predicted defect clusters and the defect clusters actually observed during the testing or operational phase are an important part of the risk analysis that is done to guide testing efforts accordingly. This does not mean that there are fewer defects in the other components, it is just that within testing, we focus on the most user-relevant paths, and that's where we find most of the defects.

**Pareto Rule** states that a small number of causes cause a large number of effects. In testing terminology, for example, it could be translated like this: about 20% of components contain about 80% of defects.

A rational approach based on the principle would require focusing even more on component A rather than component B, because it is more likely to contain defects. This is a good example of how risk analysis can be used to prioritize testing.

### 5. Tests wear out. (or the "Pesticide Paradox")

If the same tests are repeated again and again, then after changes that lead to the removal of detected defects, no more new defects are found.

Test cases must be *regularly reviewed and modified*. Moreover, in order to test new or corrected parts of the system under test, new tests must be created or ran with a different set of test data.

Unmodified tests lose their ability to detect defects over time. Sometimes, for example, because it allows us to confirm that the number of defects associated with regression testing is small (in the case of regression testing, we rather care that the tests always pass).

### 6. Testing is context dependent

This is a fairly *obvious* principle: testing should be done differently in different situations. We pay attention to something else when testing *life-critical* systems, something else when testing banking systems, as here, the most important part is the functional accuracy. In other cases, we may focus on performance, security, or usability.

### 7. Absence-of-errors fallacy

Some organizations still expect testers to be able to run all possible tests and detect all possible defects, but principles (1) and (2) show that this is impossible. It is also wrong to believe that simply finding and fixing a large number of defects will ensure successful system implementation because even a defect-free application (correct verification) may not meet user user requirements (incorrect validation).

This principle says that within the test process, verification alone is not enough, you still need validation, by which we make sure that the program meets the customer's requirements, and not just the technical assumptions that the project team made based on the requirements. We can create a perfect defect-free product that is completely useless from the user's point of view.

## 1.4 Test Activities, Testware, and Test Roles

### 1.4.1 Test Activities and Tasks

There is no one-size-fits all software testing process. But, there are typical and essential test activities necessary to achieve the established goals.

Which test activities are included in the test process, how they are implemented, and when they take place are usually defined in the organization's test strategy or test approach as part of test planning for a specific situation.

It is a good practice to define measurable **coverage** criteria for the test basis (for each test level or test type under consideration). In practice, these can act as so-called key performance indicators (KPIs) that favor the performance of specific activities and allow the test team to demonstrate the achievement of test objectives (e.g., coverage criteria may require at least one test for each test basis item).

The test process may or may not be formally defined. In typical situations, it consists of the following groups of activities:

1. Test planning
2. Test monitoring and test control
3. Test analysis
4. Test design
5. Test implementation
6. Test execution
7. Test completion

Contextual factors that affect the selection of an organization's test process include:

- SDLC and project methodologies used
- Test levels and test types considered
- Product risks and project risks
- Business domain
- Contractual and regulatory requirements
- Operational limitations
  - Budgets and resources
  - Schedules
- Complexity of the domain
- Test policy and practices of the organization
- Required internal and external norms/standards

#### Test Planning

**Test Planning** involves defining test objectives and the test approach to achieve them within the constraints imposed by the context. Typical test planning activities include:

- Defining test objectives
- Identifying the test activities needed to fulfill the project's mission and meet the test objectives
- Defining an approach to achieving test objectives within the limits set by the context
- Determining appropriate test techniques and test tasks
- Formulating a test execution schedule
- Defining metrics

Test plans should be revised based on feedback from test monitoring and test control activities. Test planning should be an ongoing activity.

#### Test Monitoring and Test Control

**Test monitoring** is the continuous comparison of actual and planned test progress using metrics specifically defined for this purpose in the test plan. **Test control** is the proactive taking of actions that are necessary to achieve the objectives set in the test plan (taking into account its possible updates). These actions are taken on the basis of monitoring information.

Progress against the test plan is communicated to stakeholders in written or verbal test progress reports which include any noteworthy deviations from the plan as well as testing impediments and related workarounds.

An element that supports test monitoring and control is the evaluation of exist criteria (often called Definition of Done, DoD, in an agile approach) from the test plan. Includes:

- Checking the test results and test logs against a specified coverage criteria
- Estimating the quality level of a component or system, based on the test results and test logs
- Determining whether further tests are necessary (if the tests performed so far do not achieve the original product risk coverage level; this involves writing and executing additional tests)
- Informing stakeholders about the progress of the test plan
- Writing test progress reports

#### Test Analysis

The task of **test analysis** is to look at the **test basis** and analyze it to identify testable features, define the associated test conditions, and determine "what to test" (in terms of measurable coverage criteria). General test objectives are transformed into specific test conditions. 

Test basis refers to any documentation or information that describes how software should work and serves as a foundation or reference for designing and executing test cases. Common examples of test basis include:

- Requirements specification
- Design specification
- Use cases
- User stories
- Source code
- Business rules

A **test condition** is any kind of property, feature, or attribute of the software that can be *checked* using tests. Test conditions are initial ideas for testing. They typically do not contain expected results. For example, in the case of testing an ATM, the following test conditions can be defined: "verify that the ATM correctly recognizes payment cards," "verify that the ATM accepts the correct PIN code entered by the user," "verify that the user can print the account balance," etc.

Test analysis activities verify that the requirements:

- Are consistent
- Are correctly expressed
- Are complete
- Are testable (are suitable for deriving acceptance criteria)
- Are ready for starting developing the software (the Definition of Ready, DoR)
- Do not need further grooming and thus can be used as a source for estimation
- Properly reflect the needs of the customers, users, and other stakeholders

Typical test analysis activities include:

- Familiarizing with the test basis that defines the desired functional and non-functional behavior of a component or a system
- Analysis of design and implementation information incl. diagrams, docs
- Analysis of the implementation itself
- Analysis of risk analysis reports
- Assessing testability of the test basis to identify common types of defects (ambiguities, omissions, inconsistencies, inaccuracies, contradictions, redundant instructions, etc.)
- Identifying the features and the feature sets to be tested
- Defining test conditions for individual features and prioritizing them based on test basis analysis
- Creating bidirectional traceability between test basis elements and their associated test conditions

Use of black-box, white-box, and experience-based test techniques can be useful as part of test analysis to reduce the likelihood of missing important test conditions and to define more precise and accurate test conditions.

More formal test conditions are often represented as so called test models (e.g., state transition diagrams, decision tables, control flow diagrams)

Identifying defects in the test basis as a result of test analysis is an important benefit, especially when a separate review of the test basis is not conducted. Test analysis activities can not only verify that requirements are consistent, properly expressed, and complete but also verify that requirements properly address the needs of customers, users, and other stakeholders.

#### Test Design

Here, test conditions are transformed into **test cases** at a high (logical) level. Test design answers the question of "how to test."

Test designs precedes test implementation, but it is also important to distinguish between the two activities. By designing tests before test implementation, you can identify defects in the test design, thus, reducing the risk of wasted implementation time and effort.

Test design inclues:

- Designing high-level test cases and prioritizing them
- Identifying the necessary test data
- Identifying the requirements for the test environment
- Identifying any necessary tools and infrastructure elements
- Creating bidirectional traceability between test basis, test conditions, test cases, and test procedures (expanding the traceability matrix)
- Identifying defects in the test basis

#### Test Implementation
