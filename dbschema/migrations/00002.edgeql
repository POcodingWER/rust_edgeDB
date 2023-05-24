CREATE MIGRATION m1weaktt34b7hmvgyk2pkpqzt76xcxr2uno5en7dqtlxtczs4mlgcq
    ONTO m1sms7szr3myvxfcgunwxeurcgfwppuvtton353vysjmc2hdkedi3q
{
  ALTER TYPE default::Musician {
      DROP PROPERTY band_name;
  };
  ALTER TYPE default::Musician RENAME TO default::Person;
  CREATE ABSTRACT TYPE default::Content {
      CREATE MULTI LINK actors -> default::Person {
          CREATE PROPERTY character_name -> std::str;
      };
      CREATE REQUIRED PROPERTY title -> std::str;
  };
  CREATE TYPE default::Account {
      CREATE MULTI LINK watchlist -> default::Content;
      CREATE REQUIRED PROPERTY username -> std::str {
          CREATE CONSTRAINT std::exclusive;
      };
  };
  CREATE TYPE default::Movie EXTENDING default::Content {
      CREATE PROPERTY release_year -> std::int32;
  };
  CREATE TYPE default::Show EXTENDING default::Content;
  ALTER TYPE default::Person {
      CREATE LINK filmography := (.<actors[IS default::Content]);
  };
  CREATE TYPE default::Season {
      CREATE REQUIRED LINK show -> default::Show;
      CREATE REQUIRED PROPERTY number -> std::int32;
  };
  ALTER TYPE default::Show {
      CREATE PROPERTY num_seasons := (std::count(.<show[IS default::Season]));
  };
};
