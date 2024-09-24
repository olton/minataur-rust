
const log = (...rest) => console.log(datetime().format(config.datetime_format), ...rest)
const info = (...rest) => console.info(datetime().format(config.date_format), ...rest)
const error = (...rest) => console.error(datetime().format(config.time_format), ...rest)