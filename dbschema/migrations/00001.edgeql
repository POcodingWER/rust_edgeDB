CREATE MIGRATION m1sms7szr3myvxfcgunwxeurcgfwppuvtton353vysjmc2hdkedi3q
    ONTO initial
{
  CREATE FUTURE nonrecursive_access_policies;
  CREATE TYPE default::Musician {
      CREATE PROPERTY band_name -> std::str;
      CREATE REQUIRED PROPERTY name -> std::str;
  };
};
