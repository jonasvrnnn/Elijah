CREATE TRIGGER AFTER_INSERT_PROJECTS
AFTER
INSERT
    ON PROJECTS
    WHEN NEW.draft = 1
    AND EXISTS(
        SELECT
            1
        FROM
            PROJECTS
        WHERE
            id = NEW.id
            AND draft = 0
    ) BEGIN
INSERT INTO
    PROJECT_COMPANIES(
        project_id,
        draft,
        company_name,
        show_in_carousel,
        introduction,
        header_photo,
        header_photo_copyright,
        banner_photo,
        banner_photo_copyright,
        thumbnail,
        weight,
        custom_content,
        custom_lightbox,
        visible
    )
SELECT
    project_id,
    1,
    company_name,
    show_in_carousel,
    introduction,
    header_photo,
    header_photo_copyright,
    banner_photo,
    banner_photo_copyright,
    thumbnail,
    weight,
    custom_content,
    custom_lightbox,
    visible
FROM
    PROJECT_COMPANIES
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    CORE_NUMBERS(
        id,
        project_id,
        title,
        number,
        draft
    )
SELECT
    id,
    project_id,
    title,
    number,
    1
FROM
    CORE_NUMBERS
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    PROJECT_PARTIES(
        project_id,
        draft,
        party_name,
        type
    )
SELECT
    project_id,
    1,
    party_name,
    type
FROM
    PROJECT_PARTIES
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    PROJECT_TMS(
        project_id,
        draft,
        tm_name,
        type
    )
SELECT
    project_id,
    1,
    tm_name,
    type
FROM
    PROJECT_TMS
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    PROJECT_INDUSTRIES(
        project_id,
        draft,
        industry
    )
SELECT
    project_id,
    1,
    industry
FROM
    PROJECT_INDUSTRIES
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    PROJECT_TAGS(
        project_id,
        tag,
        draft
    )
SELECT
    project_id,
    tag,
    1
FROM
    PROJECT_TAGS
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    PROJECT_CONTENT(
        project_id,
        company_name,
        id,
        previous_entry,
        draft,
        text,
        image,
        image_copyright,
        quote,
        quote_small
    )
SELECT
    project_id,
    company_name,
    id,
    previous_entry,
    1,
    text,
    image,
    image_copyright,
    quote,
    quote_small
FROM
    PROJECT_CONTENT
WHERE
    project_id = NEW.id
    AND draft = 0;

INSERT INTO
    IMAGES(
        project_id,
        company_name,
        id,
        draft,
        image,
        image_copyright,
        alt
    )
SELECT
    project_id,
    company_name,
    id,
    1,
    image,
    image_copyright,
    alt
FROM
    IMAGES
WHERE
    project_id = NEW.id
    AND draft = 0;

END;

CREATE TRIGGER AFTER_UPDATE_PROJECTS
AFTER
UPDATE
    ON PROJECTS
    WHEN NEW.draft = 0
    AND OLD.draft = 1 BEGIN
UPDATE
    PROJECT_COMPANIES
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    CORE_NUMBERS
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    PROJECT_PARTIES
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    PROJECT_TMS
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    PROJECT_INDUSTRIES
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    PROJECT_CONTENT
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    IMAGES
SET
    draft = 0
WHERE
    project_id = NEW.id;

UPDATE
    PROJECT_TAGS
SET
    draft = 0
WHERE
    project_id = NEW.id;

END;