CREATE TABLE users
(
    id                  serial PRIMARY KEY,
    username            text                  NOT NULL UNIQUE,
    password            text                  NOT NULL,
    is_admin            boolean DEFAULT false NOT NULL,
    is_trusted          boolean DEFAULT false NOT NULL,
    works_for           integer,
    email               text                  NOT NULL,
    can_edit_departures boolean DEFAULT false NOT NULL
);

CREATE TABLE contributions
(
    id              bigserial PRIMARY KEY,
    author_id       integer                  NOT NULL REFERENCES users (id),
    change          jsonb                    NOT NULL,
    submission_date timestamp with time zone NOT NULL,
    evaluator_id    integer REFERENCES users (id),
    evaluation_date timestamp with time zone,
    accepted        boolean,
    comment         character varying
);

CREATE TABLE changelog
(
    id              bigserial PRIMARY KEY,
    author_id       integer                                            NOT NULL REFERENCES users (id),
    changes         jsonb                                              NOT NULL,
    datetime        timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    contribution_id bigint REFERENCES contributions (id)
);

CREATE TABLE audit_log
(
    id       bigserial PRIMARY KEY,
    user_id  integer                                            NOT NULL REFERENCES users (id),
    datetime timestamp with time zone DEFAULT clock_timestamp() NOT NULL,
    action   jsonb                                              NOT NULL,
    addr     cidr                                               NOT NULL
);

CREATE TABLE regions
(
    id       serial PRIMARY KEY,
    name     text NOT NULL,
    geometry jsonb
);

CREATE TABLE municipalities
(
    id   serial PRIMARY KEY,
    name text    NOT NULL,
    zone integer NOT NULL
);

CREATE TABLE parishes
(
    id           serial PRIMARY KEY,
    name         text                                  NOT NULL,
    short_name   text                                  NOT NULL,
    municipality integer                               NOT NULL REFERENCES municipalities (id),
    polygon      text,
    geojson      jsonb                                 NOT NULL,
    dicofre      character(6) DEFAULT '000000'::bpchar NOT NULL
);

CREATE TABLE operators
(
    id         serial PRIMARY KEY,
    name       text NOT NULL,
    tag        text NOT NULL,
    validation jsonb
);

ALTER TABLE ONLY users
    ADD CONSTRAINT employee_fkey FOREIGN KEY (works_for) REFERENCES operators (id) NOT VALID;


CREATE TABLE operator_calendars
(
    id          serial PRIMARY KEY,
    operator_id integer NOT NULL REFERENCES operators (id),
    name        text    NOT NULL,
    calendar    jsonb   NOT NULL
);

CREATE TABLE vehicle
(
    id                  serial PRIMARY KEY,
    operator_id         integer NOT NULL REFERENCES operators (id),
    name                text    NOT NULL,
    quantity            integer NOT NULL,
    bench_seats         integer NOT NULL,
    foot_seats          integer NOT NULL,
    has_ac              boolean NOT NULL,
    has_wheelchair_ramp boolean NOT NULL,
    has_bicycle_rack    boolean NOT NULL,
    has_wifi            boolean NOT NULL,
    usb_outlets         integer NOT NULL
);

CREATE TABLE stops
(
    id                        serial PRIMARY KEY,
    name                      text                                              NOT NULL,
    short_name                text,
    -- If the name is ours or has been externally sourced (needed to make the name NON NULL)
    is_name_overridden        boolean,
    locality                  text,
    street                    text,
    door                      text,
    parish                    integer REFERENCES parishes (id),
    lon                       double precision                                  NOT NULL,
    lat                       double precision                                  NOT NULL,
    vehicle_lat               double precision,
    vehicle_lon               double precision,
    notes                     text,
    updater                   integer                  DEFAULT 1                NOT NULL,
    update_date               timestamp with time zone DEFAULT now()            NOT NULL,
    tags                      text[]                   DEFAULT ARRAY []::text[] NOT NULL,
    accessibility_meta        jsonb                    DEFAULT '{}'::jsonb      NOT NULL,
    verification_date         timestamp with time zone,
    verification_level        smallint                 DEFAULT 0                NOT NULL,
    service_check_date        date,
    infrastructure_check_date date,
    verified_position         boolean                  DEFAULT false            NOT NULL,
    survey_method             int,

-- TODO Rename this to osm_id later on (going to be a breaking change)
    external_id               text                                              NOT NULL,
-- TODO Rename this to osm_deleted later on (going to be a breaking change)
    deleted_upstream          boolean                  DEFAULT false            NOT NULL,
    osm_name                  text,
    osm_lon                   double precision,
    osm_lat                   double precision,
    osm_author                text,
    osm_differs               boolean                  DEFAULT false            NOT NULL,
    osm_sync_time             timestamp with time zone,
    osm_version               integer                  DEFAULT 0                NOT NULL,
    osm_map_quality           boolean,
    osm_history               jsonb                    DEFAULT '{}'::jsonb      NOT NULL
);

CREATE TABLE route_types
(
    id               serial PRIMARY KEY,
    operator         integer REFERENCES operators (id) ON DELETE RESTRICT,
    name             text,
    zapping_cost     integer                                   NOT NULL,
    board_cost       integer                                   NOT NULL,
    multi_trip       boolean      DEFAULT false                NOT NULL,
    badge_text_color character(7) DEFAULT '#000000'::character NOT NULL,
    badge_bg_color   character(7) DEFAULT '#FFFFFF'::character NOT NULL
);

CREATE TABLE routes
(
    id             serial PRIMARY KEY,
    code           text,
    name           text                                  NOT NULL,

    operator       integer                               NOT NULL,
    active         boolean                               NOT NULL,
    type           integer                               NOT NULL REFERENCES route_types (id),
    official_name  text      DEFAULT ''::text            NOT NULL,
    municipalities integer[] DEFAULT ARRAY []::integer[] NOT NULL,
    parishes       integer[] DEFAULT ARRAY []::integer[] NOT NULL,
    main_subroute  integer,
    validation     jsonb,

    -- TODO consider deprecating
    circular       boolean                               NOT NULL
);

CREATE TABLE subroutes
(
    id          serial PRIMARY KEY,
    route       integer                    NOT NULL REFERENCES routes (id),
    circular    boolean DEFAULT false      NOT NULL,

    -- Cached fields
    polyline    text,

    -- TODO consider dropping
    flag        text                       NOT NULL,

    "group"     integer                    NOT NULL,
    headsign    text                       NOT NULL,
    origin      text                       NOT NULL,
    destination text                       NOT NULL,
    via         jsonb   DEFAULT '[]'::json NOT NULL,

    validation  jsonb
);

ALTER TABLE ONLY routes
    ADD CONSTRAINT main_subroute_fkey FOREIGN KEY (main_subroute) REFERENCES subroutes (id) NOT VALID;

CREATE TABLE subroute_stops
(
    subroute     integer  NOT NULL REFERENCES subroutes (id),
    stop         integer  NOT NULL REFERENCES stops (id),
    idx          smallint NOT NULL,
    time_to_next integer
);

CREATE UNIQUE INDEX unique_subroutestops ON subroute_stops USING btree (subroute, idx);

CREATE TABLE departures
(
    id          serial PRIMARY KEY,
    subroute    integer  NOT NULL REFERENCES subroutes (id),
    calendar_id integer  NOT NULL REFERENCES operator_calendars (id),
    "time"      smallint NOT NULL
);


CREATE TABLE stop_operators
(
    stop_id       integer NOT NULL REFERENCES stops (id),
    operator_id   integer NOT NULL REFERENCES operators (id),
    stop_ref      text,
    official_name text,
    source        text    NOT NULL,
    PRIMARY KEY (stop_id, operator_id)
);

CREATE TABLE region_operators
(
    region_id   integer NOT NULL REFERENCES regions (id),
    operator_id integer NOT NULL REFERENCES operators (id),
    PRIMARY KEY (region_id, operator_id)
);

CREATE TABLE region_routes
(
    region_id integer NOT NULL REFERENCES regions (id),
    route_id  integer NOT NULL REFERENCES routes (id),
    PRIMARY KEY (region_id, route_id)
);

CREATE TABLE region_stops
(
    region_id integer NOT NULL REFERENCES regions (id),
    stop_id   integer NOT NULL REFERENCES stops (id),
    PRIMARY KEY (region_id, stop_id)
);


CREATE TABLE stop_pics
(
    id                serial PRIMARY KEY,
    original_filename text                              NOT NULL,
    sha1              character(40)                     NOT NULL,
    public            boolean  DEFAULT false            NOT NULL,
    sensitive         boolean  DEFAULT true             NOT NULL,
    tagged            boolean  DEFAULT false            NOT NULL,
    uploader          integer                           NOT NULL REFERENCES users (id),
    upload_date       timestamp with time zone          NOT NULL,
    capture_date      timestamp without time zone,
    width             integer                           NOT NULL,
    height            integer                           NOT NULL,
    lon               double precision,
    lat               double precision,
    camera_ref        text,
    notes             text,
    update_date       text,
    updater           integer REFERENCES users (id),
    quality           smallint DEFAULT 0                NOT NULL,
    tags              text[]   DEFAULT ARRAY []::text[] NOT NULL,
    attrs             text[]   DEFAULT ARRAY []::text[] NOT NULL
);

CREATE TABLE stop_pic_stops
(
    pic   integer                         NOT NULL REFERENCES stop_pics (id),
    stop  integer                         NOT NULL REFERENCES stops (id),
    attrs text[] DEFAULT ARRAY []::text[] NOT NULL,
    PRIMARY KEY (pic, stop)
);

CREATE TABLE panoramas
(
    id                serial PRIMARY KEY,
    original_filename text                     NOT NULL,
    sha1              character(40)            NOT NULL,
    lon               double precision,
    lat               double precision,
    stop_id           integer REFERENCES stops (id),
    uploader          integer                  NOT NULL REFERENCES users (id),
    upload_date       timestamp with time zone NOT NULL,
    capture_date      timestamp without time zone,
    sensitive         boolean DEFAULT false    NOT NULL
);

CREATE TABLE issues
(
    id                  serial PRIMARY KEY,
    title               text                     NOT NULL,
    message             text                     NOT NULL,
    creation            timestamp with time zone NOT NULL,
    category            text                     NOT NULL,
    geojson             jsonb,
    lat                 double precision,
    lon                 double precision,
    state               text                     NOT NULL,
    state_justification text,
    impact              integer                  NOT NULL
);

CREATE TABLE issue_stops
(
    issue_id integer NOT NULL REFERENCES issues (id),
    stop_id  integer NOT NULL REFERENCES stops (id),
    PRIMARY KEY (issue_id, stop_id)
);

CREATE TABLE issue_routes
(
    issue_id integer NOT NULL REFERENCES issues (id),
    route_id integer NOT NULL REFERENCES routes (id),
    PRIMARY KEY (issue_id, route_id)
);

CREATE TABLE issue_operators
(
    issue_id    integer NOT NULL REFERENCES issues (id),
    operator_id integer NOT NULL REFERENCES operators (id),
    PRIMARY KEY (issue_id, operator_id)
);

CREATE TABLE issue_pics
(
    issue_id integer NOT NULL REFERENCES issues (id),
    pic_id   integer NOT NULL REFERENCES stop_pics (id),
    PRIMARY KEY (issue_id, pic_id)
);

CREATE TABLE abnormalities
(
    id            bigserial PRIMARY KEY,
    summary       text                  NOT NULL,
    message       text                  NOT NULL,
    from_datetime time with time zone,
    to_datetime   time with time zone,
    geojson       jsonb,
    mark_resolved boolean DEFAULT false NOT NULL
);

CREATE TABLE abnormality_operators
(
    abnormality_id bigint  NOT NULL,
    operator_id    integer NOT NULL,
    PRIMARY KEY (abnormality_id, operator_id)
);

CREATE TABLE abnormality_routes
(
    abnormality_id integer NOT NULL REFERENCES abnormalities (id),
    route_id       integer NOT NULL REFERENCES routes (id),
    PRIMARY KEY (abnormality_id, route_id)
);

CREATE TABLE tickets
(
    id          bigserial PRIMARY KEY,
    title       bit varying                       NOT NULL,
    message     text                              NOT NULL,
    datetime    time with time zone               NOT NULL,
    operator_id integer REFERENCES operators (id) NOT NULL,
    user_id     integer                           NOT NULL,
    status      smallint                          NOT NULL
);

CREATE TABLE ticket_comments
(
    id        bigint PRIMARY KEY,
    ticket_id bigint  NOT NULL REFERENCES tickets (id),
    message   text    NOT NULL,
    user_id   integer NOT NULL REFERENCES users (id)
);

CREATE TABLE news_items
(
    id          serial PRIMARY KEY,
    operator_id integer REFERENCES operators (id),
    summary     text                     NOT NULL,
    content     text                     NOT NULL,
    datetime    timestamp with time zone NOT NULL,
    geojson     jsonb,
    visible     boolean                  NOT NULL
);